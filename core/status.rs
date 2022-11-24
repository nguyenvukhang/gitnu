use crate::line::{uncolor, Line};
use crate::{lines, App};
use std::io::{LineWriter, Write};
use std::path::PathBuf;
use std::process::Stdio;

struct State {
    seen_untracked: bool,
    count: usize,
}

fn normal(state: &mut State, path: &PathBuf, line: String) -> Option<PathBuf> {
    state.seen_untracked |= line.starts_with("Untracked files:");
    if !line.starts_with('\t') {
        println!("{}", line);
        return None;
    }
    println!("{}{}", state.count, line);
    state.count += 1;
    let mut line: &[u8] = &uncolor(&line);
    let line = &mut line;
    line.after_last(b'\t');
    let is_rename = line.starts_with(b"renamed:");
    if !state.seen_untracked {
        line.after_last(b':');
    }
    line.trim_left(b' ');
    if is_rename {
        line.after_first_sequence(b"->");
        line.trim_left_while(|v| v.is_ascii_whitespace());
    }
    if line.is_empty() {
        return None;
    }
    std::str::from_utf8(line).map(|v| path.join(v)).ok()
}

fn short(state: &mut State, path: &PathBuf, line: String) -> PathBuf {
    println!("{: <3}{}", state.count, line);
    state.count += 1;
    let line = String::from_utf8(uncolor(&line)).unwrap();
    path.join(&line[3..])
}

fn inner(app: &mut App, is_normal: bool) -> Option<()> {
    let mut git = app.cmd.stdout(Stdio::piped()).spawn().ok()?;
    let lines = lines(git.stdout.as_mut()?);
    let writer = &mut app.cache(true).map(LineWriter::new);
    let write = |line: PathBuf| {
        writer.as_mut().map(|w| writeln!(w, "{}", line.display()));
    };
    let state = &mut State { seen_untracked: false, count: 1 };
    if is_normal {
        lines.filter_map(|v| normal(state, &app.cwd, v)).for_each(write);
    } else {
        lines.map(|v| short(state, &app.cwd, v)).for_each(write);
    };
    git.wait().ok();
    writer.as_mut().and_then(|w| w.flush().ok())
}

/// Endpoint function for everything git-status related.
///
/// Runs `git status` then parses its output, enumerates it, and
/// prints it out to stdout.
pub fn status(app: &mut App, is_normal: bool) {
    inner(app, is_normal);
}
