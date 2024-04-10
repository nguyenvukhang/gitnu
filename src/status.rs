use crate::prelude::*;

use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Stdio};

/// Removes all ANSI color codes
pub fn uncolor(src: &str) -> Vec<u8> {
    let (mut b, mut j, mut on) = (src.as_bytes().to_owned(), 0, true);
    for i in 0..b.len() {
        match (on, b[i]) {
            (_, b'\x1b') => on = false,
            (true, _) => (b[j], j) = (b[i], j + 1),
            (_, b'm') => on = true,
            _ => (),
        };
    }
    b.truncate(j);
    b
}

struct State {
    seen_untracked: bool,
    count: usize,
}

/// Uses the `\t` character to differentiate between lines that
/// contain pathspecs and those that do not.
///
/// if None is returned, the line will not be added to cache.
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
    let line = line
        .rsplit_once('\t')
        .expect("There should be a tab character in the line")
        .1;

    // Example:
    // ```
    // Changes not staged for commit:
    // 1       modified:   core/status.rs
    //
    // Untracked files:
    // 2       core/line.rs
    // ```
    let (delta, pathspec) = match state.seen_untracked {
        false => line
            .split_once(':')
            .expect("There should be a `:` character in the line"),
        true => ("", line),
    };

    let pathspec = pathspec.trim_start();

    let pathspec = match delta {
        // Example:
        // ```
        // Changes to be committed:
        // 1       renamed:    README.md -> BUILD.md
        // ```
        "renamed" => pathspec
            .split_once("->")
            .expect("There should be a `->` in the line with a rename")
            .1
            .trim_start(),
        _ => pathspec,
    };

    Some(pathspec.to_string())
}

fn short(state: &mut State, line: String) -> String {
    println!("{: <3}{}", state.count, line);
    state.count += 1;
    String::from_utf8_lossy(&uncolor(&line))[3..].to_string()
}

pub fn git_status(
    mut argh: Command,
    git_dir: &PathBuf,
    git_cmd: GitCommand,
) -> Result<ExitStatus> {
    let mut git = argh.stdout(Stdio::piped()).spawn()?;

    let lines = match git.stdout.take() {
        Some(v) => BufReader::new(v).lines().filter_map(|v| v.ok()),
        None => return Ok(git.wait()?),
    };

    let cwd = argh.get_current_dir().unwrap();
    let mut cache_filepath = cwd.join(&git_dir);
    cache_filepath.push(CACHE_FILE_NAME);

    let writer = &mut LineWriter::new(File::create(&cache_filepath)?);

    // first line of the cache file is the current directory
    writeln!(writer, "{}", cwd.display()).unwrap();

    let state = &mut State { seen_untracked: false, count: 1 };

    use GitCommand::Status;

    if let Status(GitStatus::Short) = git_cmd {
        for line in lines {
            writeln!(writer, "{}", short(state, line)).unwrap();
        }
    } else if let Status(GitStatus::Normal) = git_cmd {
        for line in lines {
            if let Some(v) = normal(state, line) {
                writeln!(writer, "{v}").unwrap();
            }
        }
    }

    // close the writer
    writer.flush().ok();

    Ok(git.wait()?)
}
