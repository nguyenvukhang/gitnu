use std::any::Any;
use std::collections::HashMap;
use std::io;
use std::process;
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
    ExitStatus(process::ExitStatus),
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
            (ExitStatus(lhs), ExitStatus(rhs)) => lhs == rhs,
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

pub trait ToError<T> {
    fn to_err(self) -> Result<T>;
}
impl ToError<ExitStatus> for std::result::Result<ExitStatus, io::Error> {
    fn to_err(self) -> Result<ExitStatus> {
        match self {
            Err(e) => Err(Error::Io(e)),
            Ok(v) if v.success() => Ok(v),
            Ok(v) => Err(Error::ExitStatus(v)),
        }
    }
}

pub trait ToExitCode {
    fn exitcode(self) -> ExitCode;
}

impl ToExitCode for ExitStatus {
    fn exitcode(self) -> ExitCode {
        if let Some(code) = self.code() {
            ExitCode::from((code % 256) as u8)
        } else {
            ExitCode::FAILURE
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
