use crate::prelude::*;
use crate::{App, MAX_CACHE_SIZE};

use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write};

/// Removes all ANSI color codes
pub fn uncolor(src: &str) -> Vec<u8> {
    let (mut src, mut dst) = (src.as_bytes(), vec![]);
    while !src.is_empty() {
        match src.iter().position(|v| v == &b'\x1b') {
            None => break,
            Some(i) => {
                dst.extend(&src[..i]);
                src = &src[i..];
            }
        }
        match src.iter().position(|v| v == &b'm') {
            None => break,
            Some(i) => src = &src[i + 1..],
        }
    }
    dst.extend(src);
    dst
}

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

    let line = &uncolor(&line);
    let line: &str = std::str::from_utf8(line).unwrap();
    let line = line.rsplit_once('\t')?.1;

    // Example:
    // ```
    // Changes not staged for commit:
    // 1       modified:   core/status.rs
    //
    // Untracked files:
    // 2       core/line.rs
    // ```
    let (delta, line) = match state.seen_untracked {
        true => ("", line),
        false => line.split_once(':')?,
    };

    let line = line.trim_start_matches(|v: char| v.is_ascii_whitespace());

    let line = match delta {
        // Example:
        // ```
        // Changes to be committed:
        // 1       renamed:    README.md -> BUILD.md
        // ```
        "rename" => line
            .split_once("->")?
            .1
            .trim_start_matches(|v: char| v.is_ascii_whitespace()),
        _ => line,
    };

    if line.is_empty() {
        return None;
    }

    Some(line.to_string())
}

fn short(state: &mut State, line: String) -> String {
    println!("{: <3}{}", state.count, line);
    state.count += 1;
    String::from_utf8_lossy(&uncolor(&line))[3..].to_string()
}

trait Writer {
    fn write(self, writer: &mut LineWriter<File>);
}

impl<I: Iterator<Item = String>> Writer for I {
    fn write(self, writer: &mut LineWriter<File>) {
        self.for_each(|v| {
            writeln!(writer, "{v}").unwrap();
        });
    }
}

/// Endpoint function for everything git-status related.
///
/// Runs `git status` then parses its output, enumerates it, and
/// prints it out to stdout.
pub fn status(app: &mut App, is_normal: bool) -> Result<()> {
    let mut git = app.git.spawn_piped()?;

    let lines = match git.stdout.take() {
        Some(v) => BufReader::new(v).lines().filter_map(|v| v.ok()),
        None => return git.wait().to_err().map(|_| ()),
    };
    let writer = &mut LineWriter::new(File::create(app.cache_path()?)?);

    // first line of the cache file is the current directory
    writeln!(writer, "{}", app.cwd().to_str().unwrap()).unwrap();

    // write all the files listed by `git status`
    let state = &mut State { seen_untracked: false, count: 1 };
    if is_normal {
        lines.filter_map(|v| normal(state, v)).write(writer);
    } else {
        lines.map(|v| short(state, v)).write(writer);
    };

    // close the writer
    writer.flush().ok();

    git.wait().to_err().map(|_| ())
}
