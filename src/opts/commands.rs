use crate::opts::{Commands, OpType, Opts};
use std::io::{Error, ErrorKind::Other};
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;

impl Opts {
    /// get git command loaded with cwd
    fn git_cmd(&self) -> Command {
        let mut git = Command::new("git");
        git.current_dir(&self.cwd);
        git
    }

    /// get xargs cmd loaded with cwd
    fn xargs_cmd(&self) -> Option<Command> {
        let cmd = self.xargs_cmd.as_ref()?;
        let mut cmd = Command::new(cmd);
        cmd.current_dir(&self.cwd);
        Some(cmd)
    }
}

impl Commands for Opts {
    fn cmd(&self) -> Option<Command> {
        use OpType::*;
        match self.op {
            Read | Status | Bypass => Some(self.git_cmd()),
            Xargs => self.xargs_cmd(),
        }
    }

    fn run(&self, args: Vec<PathBuf>) -> Result<ExitStatus, Error> {
        let err = Error::new(Other, "Unable to run");
        self.cmd().ok_or(err)?.args(args).spawn()?.wait()
    }
}
