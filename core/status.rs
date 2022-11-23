use crate::{lines, App};
use std::io::{LineWriter, Write};
use std::path::PathBuf;
use std::process::Stdio;

struct State {
    seen_untracked: bool,
    count: usize,
}

fn uncolor(base: &str) -> String {
    let mut base = base.as_bytes();
    let mut dst: Vec<u8> = vec![];
    while !base.is_empty() {
        match base.iter().position(|&b| b == b'\x1b') {
            None => break,
            Some(i) => {
                dst.extend(&base[..i]);
                base = &base[i..];
            }
        }
        match base.iter().position(|&b| b == b'm') {
            None => break,
            Some(i) => base = &base[i + 1..],
        }
    }
    dst.extend(base);
    String::from_utf8(dst).unwrap_or("".to_string())
}

fn normal(state: &mut State, app: &App, line: String) -> Option<PathBuf> {
    state.seen_untracked |= line.contains("Untracked files:");
    if !line.starts_with('\t') {
        println!("{}", line);
        return None;
    }
    println!("{}{}", state.count, line);
    state.count += 1;
    let f = uncolor(&line.trim_start_matches('\t'));
    let f = match state.seen_untracked {
        true => &f,
        false => f.split_once(':')?.1.trim_start(),
    };
    Some(app.cwd.join(f))
}

fn short(state: &mut State, app: &App, line: String) -> PathBuf {
    println!("{: <3}{}", state.count, line);
    state.count += 1;
    app.cwd.join(&uncolor(&line)[3..])
}

fn inner(mut app: App, is_normal: bool) -> Option<()> {
    let mut git = app.cmd.stdout(Stdio::piped()).spawn().ok()?;
    let lines = lines(git.stdout.as_mut()?);
    let writer = &mut app.cache(true).map(LineWriter::new);
    let write = |line: PathBuf| {
        writer.as_mut().map(|lw| writeln!(lw, "{}", line.display()));
    };
    let state = &mut State { seen_untracked: false, count: 1 };
    if is_normal {
        lines.filter_map(|v| normal(state, &app, v)).for_each(write);
    } else {
        lines.map(|v| short(state, &app, v)).for_each(write);
    };
    git.wait().ok();
    writer.as_mut().map(|lw| lw.flush().ok()).flatten();
    Some(())
}

/// Endpoint function for everything git-status related.
///
/// Runs `git status` then parses its output, enumerates it, and
/// prints it out to stdout.
pub fn status(app: App, is_normal: bool) {
    inner(app, is_normal);
}
