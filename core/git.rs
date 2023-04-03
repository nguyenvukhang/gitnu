use crate::command::CommandOps;
use crate::git_cmd::is_default_git_cmd;
use crate::lines;
use std::collections::HashMap;
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

/// A struct that helps to check if a &str is a git command or not,
/// aliases included.
pub struct Commander {
    aliases: HashMap<String, String>,
}

impl Commander {
    pub fn new<P: AsRef<Path>>(cwd: P) -> Self {
        Self { aliases: aliases(cwd) }
    }

    /// Gets the actual subcommand of the argument passed in.
    /// 1. arg is a default subcommand -> it will be returned
    /// 2. arg is a valid alias -> alias target will be returned
    pub fn get_subcommand<'a>(&'a self, arg: &'a str) -> Option<&'a str> {
        if let Some(sc) = self.aliases.get(arg) {
            return Some(sc);
        } else if is_default_git_cmd(arg) {
            return Some(arg);
        }
        None
    }
}

pub fn aliases<P: AsRef<Path>>(cwd: P) -> HashMap<String, String> {
    let mut git = git!(Some(cwd), ["config", "--get-regexp", "^alias\\."]);
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

/// Path to git's repository (not workspace)
///   * .git/
///   * .git/worktrees/<branch-name>/
pub fn git_dir<P: AsRef<Path>>(cwd: P) -> Option<PathBuf> {
    git!(Some(cwd), ["rev-parse", "--git-dir"]).stdout_pathbuf()
}
