use crate::git_cmd::GitCommand;
use crate::prelude::*;

use std::ffi::OsStr;
use std::path::Path;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;

#[derive(Debug)]
pub struct Git {
    cmd: Command,
    subcommand: Option<GitCommand>,
    hidden_args: Vec<usize>,
}

impl Git {
    pub fn new() -> Self {
        let mut git = Self {
            cmd: Command::new("git"),
            subcommand: None,
            hidden_args: vec![],
        };
        if atty::is(atty::Stream::Stdout) {
            git.hidden_args(["-c", "color.ui=always"]);
        }
        git
    }

    pub fn hidden_args<I, S>(&mut self, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut n = self.cmd.get_args().len();
        for arg in args {
            self.cmd.arg(arg);
            self.hidden_args.push(n);
            n += 1;
        }
    }

    /// Gets the arguments of the underlying command, filtering out
    /// the hidden ones
    pub fn get_args(&self) -> Vec<&OsStr> {
        let all_args: Vec<_> = self.cmd.get_args().collect();

        let mut hidden = self.hidden_args.clone();
        hidden.reverse();

        let n = all_args.len();
        let mut args: Vec<&OsStr> = Vec::with_capacity(n - hidden.len() + 1);

        for i in 0..n {
            if hidden.last() == Some(&i) {
                hidden.pop();
                continue;
            }
            args.push(all_args[i])
        }
        args
    }

    pub fn get_string_args(&self) -> Vec<String> {
        self.get_args()
            .into_iter()
            .filter_map(|v| v.to_str())
            .map(|v| v.to_string())
            .collect()
    }

    /// Appends an argument to the underlying command, and updates the
    /// `subcommand` field if it's blank and the new arg is a git
    /// command.
    pub fn arg(&mut self, arg: &str) {
        if let None = self.subcommand {
            if let Ok(sub) = GitCommand::try_from(arg) {
                self.subcommand = Some(sub)
            }
        }
        self.cmd.arg(arg);
    }

    pub fn args<I, S>(&mut self, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.cmd.args(args);
    }

    /// Appends an argument to the underlying command, and updates the
    /// `subcommand` field if it's blank and the new arg is a git
    /// command.
    pub fn arg_pathspec<S: AsRef<OsStr>>(&mut self, arg: S) {
        self.cmd.arg(arg);
    }

    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) {
        self.cmd.current_dir(dir.as_ref());
    }

    pub fn get_current_dir(&self) -> Option<&Path> {
        self.cmd.get_current_dir()
    }

    /// Get the git subcommand. (e.g. 'status', 'diff', 'add')
    pub fn subcommand(&self) -> Option<&GitCommand> {
        self.subcommand.as_ref()
    }

    pub fn set_subcommand(&mut self, sc: GitCommand) {
        self.subcommand = Some(sc)
    }

    pub fn status(&mut self) -> Result<()> {
        self.cmd.status().to_err().map(|_| ())
    }

    pub fn spawn_piped(&mut self) -> Result<Child> {
        Ok(self.cmd.stdout(Stdio::piped()).spawn()?)
    }
}
