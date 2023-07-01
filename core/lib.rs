use std::fs::File;
use std::io::{BufRead, BufReader, Read};
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
    cmd2: Git,

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
        let mut cmd2 = Git::new();
        cmd2.current_dir(&cwd);

        App {
            cache: Vec::with_capacity(MAX_CACHE_SIZE),
            cmd2,
            file_prefix: PathBuf::new(),
        }
    }

    /// Get the current directory of the app
    pub fn cwd(&self) -> &Path {
        // Unwrap safety is guaranteed by App::new() always
        // initializing `self.cmd` with a value.
        self.cmd2.get_current_dir().unwrap()
    }

    /// Sets the git_command of the App.
    ///
    /// It is the responsibility of the programmer to ensure that this
    /// function is only called once per invokation of gitnu.
    pub fn set_git_command(&mut self, git_command: GitCommand) {
        self.cmd2.set_subcommand(git_command);
    }

    /// Appends an argument to the final command to be ran.
    pub fn arg<S: AsRef<std::ffi::OsStr>>(&mut self, arg: S) {
        let arg = arg.as_ref();
        // TODO: fix this unwrap
        self.cmd2.arg(arg.to_str().unwrap());
    }

    /// Get a reference to the current git command detected.
    pub fn git_command(&self) -> Option<&GitCommand> {
        self.cmd2.subcommand()
    }

    /// Runs Gitnu after all parsing is complete.
    pub fn run(&mut self) -> Result<()> {
        // print command preview if GITNU_DEBUG environment variable is set
        if std::env::var("GITNU_DEBUG").is_ok() {
            eprintln!("\x1b[0;30m{}\x1b[0m", self.preview_args().join(" "));
        }
        use GitCommand as G;
        match self.cmd2.subcommand() {
            Some(G::Status(is_normal)) => status::status(self, *is_normal),
            Some(G::Version) => {
                let result = self.cmd2.status();
                println!("gitnu version {}", VERSION.unwrap_or("unknown"));
                result
            }
            _ => self.cmd2.status(),
        }
    }

    /// Returns a complete list of arguments
    pub fn preview_args(&self) -> Vec<String> {
        self.cmd2.get_string_args()
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
            Some(pathspec) => self.arg(self.file_prefix.join(pathspec)),
            None => self.arg(n.to_string()),
        });
    }

    /// Eagerly loads cache file into buffer without actually reading any lines
    /// yet. Should only be called when confirmed App's git_command is of the
    /// `Number` variant.
    fn load_cache(&mut self) -> Result<()> {
        self.cache = vec!["0".to_string()];
        // TODO: rewrite cache operaitions to all return Result
        if let Ok(file) = File::open(self.cache_path()?) {
            let mut buffer = BufReader::new(file).lines();
            let cache_cwd = PathBuf::from(buffer.next().unwrap().unwrap());
            self.file_prefix = diff_paths(cache_cwd, self.cwd()).unwrap();
            self.cache.extend(buffer.filter_map(|v| v.ok()));
        }
        Ok(())
    }
}

/// Conveniently converts either a `File` or `Output` into an iterator of
/// `String`s, dropping the invalid ones.
fn lines<R: Read>(src: R) -> impl Iterator<Item = String> {
    BufReader::new(src).lines().filter_map(|v| v.ok())
}

#[cfg(test)]
mod tests;
