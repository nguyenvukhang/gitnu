#[macro_use]
mod prelude;

#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::thread;

mod command;
mod git;
mod git_cmd;
mod parser;
mod pathdiff;
mod status;

use prelude::*;

use command::Git;
use git_cmd::GitCommand;
use pathdiff::diff_paths;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub const MAX_CACHE_SIZE: usize = 20;

/// Gitnu's running state.
#[derive(Debug)]
pub struct App {
    git: Git,

    /// Cache that came from latest run of `gitnu status`
    cache: Vec<String>,

    /// Location that `git status` was last ran on
    file_prefix: Option<PathBuf>,

    /// Location of the git REPOSITORY, not the workspace.
    git_dir: Option<PathBuf>,
}

impl App {
    /// Creates a new App instance.
    ///
    /// Preprocessing done in parallel:
    /// 1. git-dir
    /// 2. git aliases
    /// 3. read cache file
    pub fn new(cwd: PathBuf) -> Result<App> {
        let cwd_clone = cwd.clone();
        let h_git_dir = thread::spawn(|| git::git_dir(cwd_clone));
        let cwd_clone = cwd.clone();
        let h_git_aliases = thread::spawn(|| git::git_aliases(cwd_clone));

        let git_dir = h_git_dir.join()?.ok();
        let aliases = h_git_aliases.join()??;

        let mut git = Git::new(aliases);
        git.current_dir(&cwd);

        let mut app = App {
            cache: Vec::with_capacity(MAX_CACHE_SIZE),
            git,
            file_prefix: None,
            git_dir,
        };

        match app.load_cache() {
            Ok(_) => {}
            Err(e) => match e {
                Error::NotGitRepository => {}
                _ => Err(e).unwrap(),
            },
        }

        Ok(app)
    }

    /// Get the current directory of the app
    pub fn cwd(&self) -> &Path {
        // Unwrap safety is guaranteed by App::new() always
        // initializing `self.cmd` with a value.
        self.git.get_current_dir().unwrap()
    }

    /// Runs Gitnu after all parsing is complete.
    pub fn run(&mut self) -> Result<()> {
        // print command preview if GITNU_DEBUG environment variable is set
        let _ = std::env::var("GITNU_DEBUG").map(|v| match v.as_str() {
            "1" => {
                let output_args = self.git.get_string_args();
                eprintln!("\x1b[0;30m{}\x1b[0m", output_args.join(" "))
            }
            _ => {}
        });
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
        for i in start..end + 1 {
            match (&self.file_prefix, self.cache.get(i)) {
                (Some(pre), Some(pathspec)) => {
                    self.git.arg_unchecked(pre.join(pathspec))
                }
                // TODO: design a test for this match branch.
                //
                // Probably needs a repo rooted in /var and the command ran from /home
                (None, Some(pathspec)) => self.git.arg_unchecked(pathspec),
                _ => self.git.arg_unchecked(i.to_string()),
            }
        }
    }

    fn load_cache(&mut self) -> Result<()> {
        debug_assert!(self.cache.is_empty());
        let cache_path = self.cache_path()?;
        self.cache.push(0.to_string());
        let file = match File::open(cache_path) {
            Ok(v) => v,
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => return Ok(()),
                _ => Err(e)?,
            },
        };
        let mut buf = BufReader::new(file).lines().filter_map(|v| v.ok());
        let cache_cwd = PathBuf::from(buf.next().unwrap());
        self.file_prefix = diff_paths(cache_cwd, self.cwd());
        self.cache.extend(buf);
        Ok(())
    }

    pub fn debug(&self) -> Result<()> {
        git::git_aliases(self.cwd()).ok();
        Ok(())
    }
}
