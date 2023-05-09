use crate::command::CommandOps;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

macro_rules! git {
    ($args:expr) => {{
        git!(None as Option<PathBuf>, $args)
    }};
    ($cwd:expr, $args:expr) => {{
        let mut git = Command::new("git");
        git.stdout(Stdio::piped());
        if let Some(cwd) = $cwd {
            git.current_dir(cwd);
        }
        git.args($args);
        git
    }};
}

/// Path to git's repository (not workspace)
///   * .git/
///   * .git/worktrees/<branch-name>/
pub fn git_dir<P: AsRef<Path>>(cwd: P) -> Option<PathBuf> {
    git!(Some(cwd), ["rev-parse", "--git-dir"]).stdout_pathbuf()
}
