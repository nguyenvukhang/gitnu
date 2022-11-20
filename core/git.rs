use crate::git_cmd::GIT_CMD;
use crate::lines;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

type Args<'a> = &'a Vec<String>;

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

fn git<S: AsRef<OsStr>, I: IntoIterator<Item = S>>(args: I) -> Command {
    let mut c = Command::new("git");
    c.args(args);
    c
}

fn load_aliases(set: &mut HashSet<String>) -> Option<()> {
    let mut git = git(["config", "--name-only", "--get-regexp", "^alias\\."])
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;
    lines(git.stdout.as_mut()?).for_each(|v| {
        if let Some(alias) = v.strip_prefix("alias.") {
            set.insert(alias.to_string());
        }
    });
    git.wait().map(|_| ()).ok()
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
pub fn git_dir(args: Args) -> Option<PathBuf> {
    let git = git(args).args(["rev-parse", "--git-dir"]).output();
    git.ok()?.to_pathbuf()
}
