use crate::command::CommandOps;
use crate::git_cmd::GIT_CMD;
use crate::lines;
use std::collections::{HashMap, HashSet};
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

/// A struct that helps to check if a &str is a git command or not,
/// aliases included.
pub struct Commander {
    aliases: HashMap<String, String>,
    subcommands: HashSet<String>,
}

impl Commander {
    pub fn new() -> Self {
        Self { aliases: aliases(), subcommands: subcommands() }
    }

    /// Gets the actual subcommand of the argument passed in.
    /// 1. arg is a default subcommand -> it will be returned
    /// 2. arg is a valid alias -> alias target will be returned
    pub fn get_subcommand<'a>(&'a self, arg: &'a str) -> Option<&'a str> {
        if let Some(sc) = self.aliases.get(arg) {
            return Some(sc);
        } else if self.subcommands.contains(arg) {
            return Some(arg);
        }
        None
    }
}

pub fn aliases() -> HashMap<String, String> {
    let mut git = git!(["config", "--get-regexp", "^alias\\."]);
    let mut git = git.spawn().ok().expect("Unable to collect aliases.");
    let stdout = match git.stdout.take() {
        Some(v) => v,
        None => return HashMap::new(),
    };
    let mut hs = HashMap::new();
    hs.extend(lines(stdout).filter_map(|v| {
        v.strip_prefix("alias.")
            .and_then(|v| v.split_once(" "))
            .map(|v| (v.0.to_string(), v.1.to_string()))
    }));
    git.wait().ok().map(|_| ());
    hs
}

/// Get a hashset of all git subcommands from two sources
///   1. git's default subcommand list
///   2. user's git subcommand aliases
pub fn subcommands() -> HashSet<String> {
    HashSet::from_iter(GIT_CMD.iter().map(|v| v.to_string()))
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
