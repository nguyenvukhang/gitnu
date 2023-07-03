use crate::git_cmd::GitCommand;
use crate::prelude::*;

use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

fn git<P: AsRef<Path>>(cwd: Option<P>, args: &[&str]) -> Command {
    let mut git = Command::new("git");
    git.stdout(Stdio::piped());
    if let Some(cwd) = cwd {
        git.current_dir(cwd);
    }
    git.args(args);
    git
}

fn git_output<P: AsRef<Path>>(cwd: Option<P>, args: &[&str]) -> Result<Output> {
    let out = git(cwd, args).output()?;
    if !out.status.success() {
        return Err(Error::ProcessError(out.status));
    }
    Ok(out)
}

/// Path to git's repository (not workspace)
///   * .git/
///   * .git/worktrees/<branch-name>/
pub fn git_dir<P: AsRef<Path>>(cwd: P) -> Result<PathBuf> {
    let out = git_output(Some(cwd), &["rev-parse", "--git-dir"])?;
    Ok(PathBuf::from(String::from_utf8_lossy(&out.stdout).trim()))
}

/// Get a HashMap of git aliases for git commands
pub fn git_aliases<P: AsRef<Path>>(cwd: P) -> Result<HashMap<String, String>> {
    let mut git = git(Some(cwd), &["config", "--get-regexp", "^alias."]);
    let mut git = git.spawn()?;

    let lines = match git.stdout.take() {
        Some(v) => BufReader::new(v).lines().filter_map(|v| v.ok()),
        None => Err(Error::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            "Can't get a handle to child stdout",
        )))?,
    };

    let ht = lines
        .filter_map(|v| {
            let (k, v) = v[6..].split_once(' ')?;
            if GitCommand::is_command(v) {
                Some((k.to_string(), v.to_string()))
            } else {
                None
            }
        })
        .collect();

    println!("OUTPUT -> {ht:?}");
    Ok(ht)
}
