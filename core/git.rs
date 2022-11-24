use crate::command::CommandOps;
use crate::git_cmd::GIT_CMD;
use std::collections::HashSet;
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

fn load_aliases(set: &mut HashSet<String>) {
    let mut git = git!(["config", "--name-only", "--get-regexp", "^alias\\."]);
    let output = git.stdout_string();
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
    git!(args, ["rev-parse", "--git-dir"]).stdout_pathbuf()
}
