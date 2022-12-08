use crate::command::CommandOps;
use crate::git_cmd::GIT_CMD;
use crate::lines;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Stdio};

macro_rules! git {
    ($($arg:tt),*) => {{
        let mut git = Command::new("git");
        git.stdout(Stdio::piped());
        $(git.args($arg);)*
        git
    }};
}

fn load_aliases(set: &mut HashSet<String>) -> Option<()> {
    let mut git = git!(["config", "--name-only", "--get-regexp", "^alias\\."]);
    let mut git = git.spawn().ok()?;
    let stdout = match git.stdout.take() {
        Some(v) => v,
        None => return git.wait().ok().map(|_| ()),
    };
    lines(stdout).for_each(|v| match v.strip_prefix("alias.") {
        Some(v) => {
            set.insert(v.to_string());
        }
        None => (),
    });
    git.wait().ok().map(|_| ())
}

/// Get a hashset of all git subcommands from two sources
///   1. git's default subcommand list
///   2. user's git subcommand aliases
pub fn subcommands() -> HashSet<String> {
    let mut set = HashSet::from_iter(GIT_CMD.iter().map(|v| v.to_string()));
    load_aliases(&mut set);
    set
}

/// Path to git's repository (not workspace)
///   * .git/
///   * .git/worktrees/<branch-name>/
pub fn git_dir<S, I>(args: I) -> Option<PathBuf>
where
    S: AsRef<OsStr>,
    I: Iterator<Item = S>,
{
    git!(args, ["rev-parse", "--git-dir"]).stdout_pathbuf()
}
