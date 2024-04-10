use std::ffi::OsStr;
use std::process::{Command, ExitStatus};

use crate::prelude::{Error, Result};

fn st<S: AsRef<OsStr>>(x: S) -> String {
    x.as_ref().to_string_lossy().to_string()
}

pub trait ArgHolder {
    fn add_arg<S: AsRef<OsStr>>(&mut self, arg: S);

    fn add_args<I, S: AsRef<OsStr>>(&mut self, args: I)
    where
        I: IntoIterator<Item = S>,
    {
        args.into_iter().for_each(|v| self.add_arg(v));
    }

    fn get_string_args(&self) -> Vec<String>;

    fn run(&mut self) -> Result<ExitStatus> {
        Err(Error::NotImplemented)
    }
}

impl ArgHolder for Vec<String> {
    fn add_arg<S: AsRef<OsStr>>(&mut self, arg: S) {
        self.push(st(arg))
    }

    fn get_string_args(&self) -> Vec<String> {
        self.clone()
    }
}

impl ArgHolder for Command {
    fn add_arg<S: AsRef<OsStr>>(&mut self, arg: S) {
        self.arg(arg);
    }

    fn get_string_args(&self) -> Vec<String> {
        self.get_args().map(|v| v.to_string_lossy().to_string()).collect()
    }

    fn run(&mut self) -> Result<ExitStatus> {
        Ok(self.status()?)
    }
}
