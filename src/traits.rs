use std::ffi::OsStr;
use std::process::{Command, ExitStatus};

use crate::prelude::{Error, Result};

pub trait ArgHolder {
    fn add_arg<S: AsRef<OsStr>>(&mut self, arg: S);

    fn add_args<I, S: AsRef<OsStr>>(&mut self, args: I)
    where
        I: IntoIterator<Item = S>,
    {
        args.into_iter().for_each(|v| self.add_arg(v));
    }

    fn run(&mut self) -> Result<ExitStatus> {
        Err(Error::NotImplemented)
    }
}

impl ArgHolder for Vec<String> {
    fn add_arg<S: AsRef<OsStr>>(&mut self, arg: S) {
        self.push(arg.as_ref().to_string_lossy().to_string())
    }
}

impl ArgHolder for Command {
    fn add_arg<S: AsRef<OsStr>>(&mut self, arg: S) {
        self.arg(arg);
    }

    fn run(&mut self) -> Result<ExitStatus> {
        Ok(self.status()?)
    }
}
