use std::fs::File;
use std::io::Lines;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::process::Command;

mod cache;
mod error;
mod git;
mod git_cmd;
mod line;
mod parser;
mod pathdiff;
mod status;

pub use error::GitnuError;
use git_cmd::GitCommand;
pub use parser::parse;

use cache::Cache;
use error::ToGitnuError;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

/// Gitnu's running state.
#[derive(Debug)]
pub struct App {
    /// Controls main flow (read/write/which parser to use)
    git_command: Option<GitCommand>,

    /// The command that will be ran once all processing is complete.
    cmd: Command,

    /// Cache that came from latest run of `gitnu status`
    cache: Vec<String>,
    buffer: Option<Lines<BufReader<File>>>,
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
            cache: vec![],
            cmd,
            buffer: None,
            file_prefix: PathBuf::new(),
        }
    }

    /// Get the current directory of the app
    pub fn cwd(&self) -> &Path {
        // Unwrap safety is guaranteed by App::new() always initializing
        // `self.cmd` with a value
        self.cmd.get_current_dir().unwrap()
    }

    /// Sets the git_command of the App.
    pub fn set_git_command(&mut self, git_command: GitCommand) {
        self.git_command = Some(git_command);
    }

    /// Appends an argument to the final command to be ran.
    pub fn arg<S: AsRef<std::ffi::OsStr>>(&mut self, arg: S) {
        self.cmd.arg(arg);
    }

    /// Runs Gitnu after all parsing is complete.
    pub fn run(&mut self) -> Result<(), GitnuError> {
        use GitCommand as G;
        match self.git_command {
            Some(G::Status(is_normal)) => status::status(self, is_normal),
            Some(G::Version) => {
                let result = self.cmd.status().gitnu_err().map(|_| ());
                println!("gitnu version {}", VERSION.unwrap_or("unknown"));
                result
            }
            _ => self.cmd.status().gitnu_err().map(|_| ()),
        }
    }

    pub fn git_command(&self) -> Option<&GitCommand> {
        self.git_command.as_ref()
    }

    #[cfg(test)]
    pub fn cmd(&self) -> &Command {
        &self.cmd
    }
}

/// Conveniently converts either a `File` or `Output` into an iterator of
/// `String`s, dropping the invalid ones.
fn lines<R: Read>(src: R) -> impl Iterator<Item = String> {
    BufReader::new(src).lines().filter_map(|v| v.ok())
}

#[cfg(test)]
mod tests;
