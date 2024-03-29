use std::any::Any;
use std::collections::HashMap;
use std::io;
use std::process::Command;
use std::process::ExitCode;
use std::process::ExitStatus;

pub(crate) const MAX_CACHE_SIZE: usize = 20;
pub(crate) const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const CACHE_FILE_NAME: &str = "gitnu.txt";

#[derive(Debug)]
pub enum Error {
    InvalidCache,
    NotGitCommand,
    NotGitRepository,
    Io(io::Error),
    ThreadError(Box<dyn Any + Send + 'static>),
}

impl PartialEq for Error {
    fn eq(&self, rhs: &Error) -> bool {
        let lhs = self;
        use Error::*;
        match (lhs, rhs) {
            (NotGitRepository, NotGitRepository) => true,
            (InvalidCache, InvalidCache) => true,
            (NotGitCommand, NotGitCommand) => true,
            (Io(lhs), Io(rhs)) => lhs.kind() == rhs.kind(),
            (ThreadError(_), ThreadError(_)) => true,
            _ => false,
        }
    }
}

#[macro_export]
macro_rules! error {
    ($enum:ident, $from:ty) => {
        impl From<$from> for Error {
            fn from(err: $from) -> Self {
                Self::$enum(err)
            }
        }
    };
    ($enum:ident) => {
        Err(Error::$enum)
    };
}

error!(Io, io::Error);
error!(ThreadError, Box<dyn Any + Send + 'static>);

pub type Result<T> = std::result::Result<T, Error>;
pub type Aliases = HashMap<String, String>;

pub trait ToExitCode {
    fn exitcode(self) -> ExitCode;
}

impl ToExitCode for ExitStatus {
    fn exitcode(self) -> ExitCode {
        match self.code() {
            Some(code) => ExitCode::from((code % 256) as u8),
            None => ExitCode::FAILURE,
        }
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

pub trait RealArgs {
    fn real_args(&self) -> Vec<String>;
}

impl RealArgs for Command {
    fn real_args(&self) -> Vec<String> {
        let x: Vec<_> =
            self.get_args().map(|v| v.to_string_lossy().to_string()).collect();
        for i in 0..x.len() {
            if x[i] == "color.ui=always" {
                let mut m = Vec::with_capacity(x.len());
                m.extend_from_slice(&x[..i - 1]);
                m.extend_from_slice(&x[i + 1..]);
                return m;
            }
        }
        x
    }
}
