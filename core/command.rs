use crate::git_cmd::GitCommand;
use crate::prelude::*;

use std::collections::HashMap;
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
    aliases: HashMap<String, String>,
}

impl Git {
    pub fn new(aliases: HashMap<String, String>) -> Self {
        let mut git = Self {
            cmd: Command::new("git"),
            subcommand: None,
            hidden_args: Vec::with_capacity(2),
            aliases,
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
    ///
    /// Returns Some() if a git command is found (None if not a git command)
    /// and Some(true) only if the git command requires cache to be loaded
    pub fn arg(&mut self, arg: &str) -> Option<bool> {
        if let None = self.subcommand {
            if let Ok(sub) = GitCommand::try_from(arg) {
                self.cmd.arg(arg);
                return Some(self.set_subcommand(sub));
            }
            if let Some(sub) = self
                .aliases
                .get(arg)
                .and_then(|v| GitCommand::try_from(v.as_str()).ok())
            {
                self.cmd.arg(arg);
                return Some(self.set_subcommand(sub));
            }
        }
        self.cmd.arg(arg);
        None
    }

    /// returns true when the git command requires cache to be loaded
    pub fn set_subcommand(&mut self, sc: GitCommand) -> bool {
        let b = sc.should_load_cache();
        self.subcommand = Some(sc);
        b
    }

    /// Append arg to self without checking if it's a git command or not
    pub fn arg_unchecked<S: AsRef<OsStr>>(&mut self, arg: S) {
        self.cmd.arg(arg);
    }

    /// Append args to self without checking if it's a git command or not
    pub fn args_unchecked<I, S>(&mut self, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.cmd.args(args);
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

    pub fn status(&mut self) -> Result<()> {
        self.cmd.status().to_err().map(|_| ())
    }

    pub fn spawn_piped(&mut self) -> Result<Child> {
        Ok(self.cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?)
    }
}
