use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn git<P: AsRef<Path>>(cwd: Option<P>, args: &[&str]) -> Command {
    let mut git = Command::new("git");
    git.stdout(Stdio::piped());
    if let Some(cwd) = cwd {
        git.current_dir(cwd);
    }
    git.args(args);
    git
}

/// Path to git's repository (not workspace)
///   * .git/
///   * .git/worktrees/<branch-name>/
pub fn git_dir<P: AsRef<Path>>(cwd: P) -> Option<PathBuf> {
    let out = git(Some(cwd), &["rev-parse", "--git-dir"]).output().ok()?;
    if !out.status.success() {
        return None;
    }
    match String::from_utf8_lossy(&out.stdout).trim() {
        v if v.is_empty() => None,
        v => Some(PathBuf::from(v)),
    }
}
