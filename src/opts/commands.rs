use crate::opts::{Commands, OpType, Opts};
use std::process::Command;

impl Opts {
    /// get git command loaded with arg_dir
    fn git_cmd(&self) -> Command {
        let mut git = Command::new("git");
        git.current_dir(&self.arg_dir);
        git
    }

    /// get xargs cmd loaded with cwd
    fn xargs_cmd(&self) -> Option<Command> {
        let cmd = self.xargs_cmd.as_ref()?;
        let mut cmd = Command::new(cmd);
        cmd.current_dir(&self.arg_dir);
        Some(cmd)
    }
}

impl Commands for Opts {
    /// get either `git` or the xargs cmd,
    /// depending on the operation type
    fn cmd(&self) -> Option<Command> {
        use OpType::*;
        match self.op {
            Read | Status | Bypass => Some(self.git_cmd()),
            Xargs => self.xargs_cmd(),
        }
    }
}
