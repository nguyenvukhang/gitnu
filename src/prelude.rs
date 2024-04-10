use std::collections::HashMap;
use std::ffi::OsStr;
use std::process::{Command, ExitCode, ExitStatus};

pub(crate) use crate::cache::Cache;
pub(crate) use crate::error::*;
pub(crate) use crate::git_cmd::*;
pub(crate) use crate::pathdiff;

pub(crate) const MAX_CACHE_SIZE: usize = 20;
pub(crate) const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const CACHE_FILE_NAME: &str = "gitnu.txt";

pub type Result<T> = std::result::Result<T, Error>;
pub type Aliases = HashMap<String, String>;

pub trait ToExitCode {
    fn to_exitcode(self) -> ExitCode;
}

impl ToExitCode for ExitStatus {
    fn to_exitcode(self) -> ExitCode {
        self.code()
            .map(|c| ExitCode::from((c % 256) as u8))
            .unwrap_or(ExitCode::FAILURE)
    }
}

#[cfg(test)]
pub fn string_vec<S, I>(v: I) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    v.into_iter().map(|v| v.as_ref().to_string()).collect()
}

/// A CLI argument holder.
///
/// Meant to be implemented by `Vec<String>` and `Command`; both of
/// which can hold a list of args. Made for testing convenience.
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
