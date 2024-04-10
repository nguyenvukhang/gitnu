use std::io::BufRead;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::error;
use crate::prelude::{Aliases, Error, Result};

/// Run a git command in a particular directory. Defaults to process's cwd.
fn sh<P: AsRef<Path>>(dir: Option<P>, args: &[&str]) -> Result<Output> {
    let mut cmd = Command::new("git");
    if let Some(dir) = dir {
        cmd.current_dir(dir);
    }
    Ok(cmd.args(args).output()?)
}

/// Path to git's repository (not workspace)
///   * .git/
///   * .git/worktrees/<branch-name>/
///
/// current_dir is intentionally not supplied as it relies on the
/// user's actual PWD or the value of git's `-C` flag, which is not
/// visible to the `git-nu` binary.
///
/// This can either be absolute or relative to cwd.
pub(crate) fn dir<P: AsRef<Path>>(cwd: P) -> Result<PathBuf> {
    let output = sh(Some(cwd), &["rev-parse", "--git-dir"])?;
    if output.stderr.starts_with(b"fatal: not a git repository") {
        return error!(NotGitRepository);
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(PathBuf::from(stdout.trim_end()))
}

pub(crate) fn aliases() -> Aliases {
    let args = ["config", "--global", "--get-regexp", "^alias."];
    match sh(None::<&str>, &args) {
        Ok(v) => Aliases::from_iter(
            v.stdout.lines().filter_map(|v| v.ok()).filter_map(|v| {
                v.get(6..) // every lines starts with "alias."
                    .and_then(|v| v.split_once(' '))
                    .map(|(k, v)| (k.to_string(), v.to_string()))
            }),
        ),
        Err(_) => Aliases::new(),
    }
}
