use crate::git_cmd::{GitCommand, GitStatus};
use crate::prelude::*;
use crate::{App, MAX_CACHE_SIZE};

use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::process::{ExitCode, Stdio};

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

impl App {
    /// Endpoint function for everything git-status related.
    ///
    /// Runs `git status` then parses its output, enumerates it, and
    /// prints it out to stdout.
    pub(crate) fn git_status(self) -> Result<ExitCode> {
        use git2::{Repository, StatusOptions};
        use GitCommand as GC;

        let mut cache_filepath = self.cwd.join(&self.git_dir);
        cache_filepath.push(CACHE_FILE_NAME);

        let writer = &mut LineWriter::new(File::create(&cache_filepath)?);

        // first line of the cache file is the current directory
        writeln!(writer, "{}", self.cwd.display()).unwrap();

        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        opts.sort_case_sensitively(true);

        let repo =
            Repository::open(&self.cwd).map_err(|_| Error::NotGitRepository)?;

        let branch = match repo.head() {
            Ok(head) => head.symbolic_target().unwrap().to_string(),
            Err(_) => {
                let x = std::fs::read(self.git_dir.join("HEAD")).unwrap();
                let x = std::str::from_utf8(&x).unwrap();
                let x = x.strip_prefix("ref:").unwrap_or(x);
                let x = x.trim();
                x.strip_prefix("refs/heads/").unwrap_or(x).to_string()
            }
        };

        let status = repo.statuses(Some(&mut opts)).unwrap();

        let mut count = 1;

        match self.git_cmd {
            Some(GC::Status(GitStatus::Short)) => {
                for item in &status {
                    println!("{: <3}{}", count, item.path().unwrap());
                }
            }
            Some(GC::Status(GitStatus::Normal)) => {
                println!("On branch {branch}\n");
            }
            _ => {}
        }
        println!("GOT HERE");


        for item in &status {}

        // close the writer
        writer.flush().ok();

        Ok(ExitCode::SUCCESS)
    }
}
