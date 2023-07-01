use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

mod git;
mod git_cmd;
mod parser;
mod pathdiff;
mod prelude;
mod status;

use prelude::*;

use git_cmd::GitCommand;
use pathdiff::diff_paths;

pub use parser::parse;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub const MAX_CACHE_SIZE: usize = 20;

/// Gitnu's running state.
#[derive(Debug)]
pub struct App {
    /// Controls main flow (read/write/which parser to use)
    git_command: Option<GitCommand>,

    /// The command that will be ran once all processing is complete.
    cmd: Command,

    /// Cache that came from latest run of `gitnu status`
    cache: Vec<String>,

    /// Location that `git status` was last ran on
    file_prefix: PathBuf,
}

impl App {
    /// Creates a new App instance.
    pub fn new(cwd: PathBuf) -> App {
        let mut cmd = Command::new("git");
        if atty::is(atty::Stream::Stdout) {
            cmd.args(["-c", "color.ui=always"]);
        }
        cmd.current_dir(&cwd);
        App {
            git_command: None,
            cache: Vec::with_capacity(MAX_CACHE_SIZE),
            cmd,
            file_prefix: PathBuf::new(),
        }
    }

    /// Get the current directory of the app
    pub fn cwd(&self) -> &Path {
        // Unwrap safety is guaranteed by App::new() always
        // initializing `self.cmd` with a value.
        self.cmd.get_current_dir().unwrap()
    }

    /// Sets the git_command of the App.
    ///
    /// It is the responsibility of the programmer to ensure that this
    /// function is only called once per invokation of gitnu.
    pub fn set_git_command(&mut self, git_command: GitCommand) {
        self.git_command = Some(git_command);
    }

    /// Appends an argument to the final command to be ran.
    pub fn arg<S: AsRef<std::ffi::OsStr>>(&mut self, arg: S) {
        self.cmd.arg(arg);
    }

    /// Get a reference to the current git command detected.
    pub fn git_command(&self) -> Option<&GitCommand> {
        self.git_command.as_ref()
    }

    #[cfg(test)]
    pub fn cmd(&self) -> &Command {
        &self.cmd
    }

    /// Runs Gitnu after all parsing is complete.
    pub fn run(&mut self) -> Result<()> {
        // print command preview if GITNU_DEBUG environment variable is set
        if std::env::var("GITNU_DEBUG").is_ok() {
            eprintln!("\x1b[0;30m{}\x1b[0m", self.preview_args().join(" "));
        }
        use GitCommand as G;
        match self.git_command {
            Some(G::Status(is_normal)) => status::status(self, is_normal),
            Some(G::Version) => {
                let result = self.cmd.status().to_err().map(|_| ());
                println!("gitnu version {}", VERSION.unwrap_or("unknown"));
                result
            }
            _ => self.cmd.status().to_err().map(|_| ()),
        }
    }

    /// Returns a complete list of arguments
    pub fn preview_args(&self) -> Vec<&str> {
        let args: Vec<_> =
            self.cmd.get_args().filter_map(|v| v.to_str()).collect();

        let mut ignore = vec![];

        // remove `-c color.ui=always` flag
        let flag_rm = args.iter().position(|&v| v == "color.ui=always");
        if let Some(v) = flag_rm {
            ignore.extend_from_slice(&[v - 1, v]);
        }

        ignore.sort();
        ignore.reverse();

        let mut preview = Vec::with_capacity(args.len() + 1);
        preview.push("git");

        for i in 0..args.len() {
            if ignore.last() == Some(&i) {
                ignore.pop();
                continue;
            }
            preview.push(args[i])
        }

        preview
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
