use crate::git_cmd::GIT_CMD;
use std::collections::HashSet;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

trait Buffable {
    fn to_pathbuf(&self) -> Option<PathBuf>;
}

impl Buffable for Output {
    fn to_pathbuf(&self) -> Option<PathBuf> {
        // don't take value if return code is a failure
        if !self.status.success() {
            return None;
        }
        // only take non-empty outputs
        match String::from_utf8_lossy(&self.stdout).trim() {
            v if v.is_empty() => None,
            v => Some(PathBuf::from(v)),
        }
    }
}

macro_rules! git {
    ($($arg:tt),*) => {{
        let mut git = Command::new("git");
        git.stdout(Stdio::piped());
        $(git.args($arg);)*
        git
    }};
}

fn stringify(v: &[u8]) -> String {
    String::from_utf8_lossy(v).parse().unwrap_or("".to_string())
}

fn load_aliases(set: &mut HashSet<String>) {
    let mut git = git!(["config", "--name-only", "--get-regexp", "^alias\\."]);
    let output = git.output().map(|v| stringify(&v.stdout)).unwrap_or_default();
    let lines = output.split_whitespace();
    lines.filter_map(|v| v.strip_prefix("alias.")).for_each(|alias| {
        set.insert(alias.to_string());
    });
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
pub fn git_dir(args: &Vec<String>) -> Option<PathBuf> {
    git!(args, ["rev-parse", "--git-dir"]).output().ok()?.to_pathbuf()
}
