use std::path::Path;
use std::process::Output;

use super::*;

/// Path to git's repository (not workspace)
///   * .git/
///   * .git/worktrees/<branch-name>/
///
/// current_dir is intentionally not supplied as it relies on the
/// user's actual PWD or the value of git's `-C` flag, which is not
/// visible to the `git-nu` binary.
pub(crate) fn dir<P: AsRef<Path>>(cwd: P) -> Result<PathBuf> {
    _dir(Some(cwd))
}

fn sh<P: AsRef<Path>>(dir: Option<P>, args: &[&str]) -> Result<Output> {
    let mut cmd = Command::new("git");
    dir.map(|v| cmd.current_dir(v));
    Ok(cmd.args(args).output()?)
}

fn _dir<P: AsRef<Path>>(base_dir: Option<P>) -> Result<PathBuf> {
    let output = git::sh(base_dir, &["rev-parse", "--git-dir"])?;
    if output.stderr.starts_with(b"fatal: not a git repository") {
        return error!(NotGitRepository);
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(PathBuf::from(stdout.trim_end()))
}

pub(crate) fn aliases() -> Aliases {
    let args = ["config", "--global", "--get-regexp", "^alias."];
    match git::sh(None::<&str>, &args) {
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
