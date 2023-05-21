use crate::line::{uncolor, Line};
use crate::{lines, App, ToGitnuError, MAX_CACHE_SIZE};
use crate::{Cache, GitnuError};
use std::fs::File;
use std::io::{LineWriter, Write};
use std::process::Stdio;

struct State {
    seen_untracked: bool,
    count: usize,
}

/// Uses the `\t` character to differentiate between lines that
/// contain pathspecs and those that do not.
fn normal(state: &mut State, line: String) -> Option<String> {
    if state.count > MAX_CACHE_SIZE {
        println!("{}", line);
        return None;
    }
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
        line.after_first(b':');
    }
    line.trim_left_while(|v| v.is_ascii_whitespace());
    if is_rename {
        line.after_first_sequence(b"->");
        line.trim_left_while(|v| v.is_ascii_whitespace());
    }
    if line.is_empty() {
        return None;
    }
    std::str::from_utf8(line).map(|v| v.to_string()).ok()
}

fn short(state: &mut State, line: String) -> String {
    println!("{: <3}{}", state.count, line);
    state.count += 1;
    String::from_utf8_lossy(&uncolor(&line))[3..].to_string()
}

trait Writer {
    fn write(self, writer: &mut Option<LineWriter<File>>);
}

impl<I: Iterator<Item = String>> Writer for I {
    fn write(self, writer: &mut Option<LineWriter<File>>) {
        if let Some(writer) = writer.as_mut() {
            self.for_each(|v| {
                writeln!(writer, "{v}").unwrap();
            });
        }
    }
}

/// Endpoint function for everything git-status related.
///
/// Runs `git status` then parses its output, enumerates it, and
/// prints it out to stdout.
pub fn status(app: &mut App, is_normal: bool) -> Result<(), GitnuError> {
    let mut git = app.cmd.stdout(Stdio::piped()).spawn().gitnu_err()?;

    let lines = match git.stdout.take() {
        Some(v) => lines(v),
        None => return git.wait().gitnu_err().map(|_| ()),
    };
    let writer = &mut app.cache_file(true).map(LineWriter::new);

    // first line of the cache file is the current directory
    if let Some(lw) = writer {
        writeln!(lw, "{}", app.cwd().to_str().unwrap()).unwrap();
    }

    // write all the files listed by `git status`
    let state = &mut State { seen_untracked: false, count: 1 };
    if is_normal {
        lines.filter_map(|v| normal(state, v)).write(writer);
    } else {
        lines.map(|v| short(state, v)).write(writer);
    };

    // close the writer if it exists
    if let Some(lw) = writer {
        lw.flush().ok();
    }
    git.wait().gitnu_err().map(|_| ())
}
