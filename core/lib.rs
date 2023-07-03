use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

mod command;
mod git;
mod git_cmd;
mod parser;
mod pathdiff;
mod prelude;
mod status;

use prelude::*;

use command::Git;
use git_cmd::GitCommand;
use pathdiff::diff_paths;

pub use parser::parse;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub const MAX_CACHE_SIZE: usize = 20;

/// Gitnu's running state.
#[derive(Debug)]
pub struct App {
    git: Git,

    /// Cache that came from latest run of `gitnu status`
    cache: Vec<String>,

    /// Location that `git status` was last ran on
    file_prefix: PathBuf,
}

impl App {
    /// Creates a new App instance.
    pub fn new(cwd: PathBuf) -> App {
        let mut cmd = Command::new("git");
        cmd.current_dir(&cwd);
        let mut git = Git::new();
        git.current_dir(&cwd);

        App {
            cache: Vec::with_capacity(MAX_CACHE_SIZE),
            git,
            file_prefix: PathBuf::new(),
        }
    }

    /// Get the current directory of the app
    pub fn cwd(&self) -> &Path {
        // Unwrap safety is guaranteed by App::new() always
        // initializing `self.cmd` with a value.
        self.git.get_current_dir().unwrap()
    }

    /// Runs Gitnu after all parsing is complete.
    pub fn run(&mut self) -> Result<()> {
        git::git_aliases(self.cwd()).ok();
        return Ok(());
        // print command preview if GITNU_DEBUG environment variable is set
        if std::env::var("GITNU_DEBUG").is_ok() {
            eprintln!("\x1b[0;30m{}\x1b[0m", self.preview_args().join(" "));
        }
        use GitCommand as G;
        match self.git.subcommand() {
            Some(G::Status(is_normal)) => status::status(self, *is_normal),
            Some(G::Version) => {
                let result = self.git.status();
                println!("gitnu version {}", VERSION.unwrap_or("unknown"));
                result
            }
            _ => self.git.status(),
        }
    }

    /// Returns a complete list of arguments
    pub fn preview_args(&self) -> Vec<String> {
        self.git.get_string_args()
    }

    /// use the path obtained from `git rev-parse --git-dir` to store the cache.
    /// this is usually the .git folder of regular repositories, and somewhere
    /// deeper for worktrees.
    fn cache_path(&self) -> Result<PathBuf> {
        let cwd = self.cwd();
        let git_dir = git::git_dir(&cwd)?;
        // git.stdout returns the git-dir relative to cwd,
        // so prepend it with current dir
        let mut path = cwd.join(git_dir);
        path.push("gitnu.txt");
        Ok(path)
    }

    /// Adds a range of files by index as arguments to the `Command` that will
    /// eventually be run.
    ///
    /// Loads files indexed [start, end] (inclusive)
    fn load_range(&mut self, start: usize, end: usize) {
        (start..end + 1).for_each(|n| match self.cache.get(n) {
            Some(pathspec) => {
                self.git.arg_unchecked(self.file_prefix.join(pathspec));
            }
            None => self.git.arg_unchecked(n.to_string()),
        });
    }

    /// Eagerly loads cache file into buffer without actually reading any lines
    /// yet. Should only be called when confirmed App's git_command is of the
    /// `Number` variant.
    fn load_cache(&mut self) -> Result<()> {
        self.cache = vec!["0".to_string()];
        if let Ok(file) = File::open(self.cache_path()?) {
            let mut buf = BufReader::new(file).lines().filter_map(|v| v.ok());
            let cache_cwd = PathBuf::from(buf.next().unwrap());
            self.file_prefix =
                diff_paths(cache_cwd, self.cwd()).unwrap_or_default();
            self.cache.extend(buf);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
