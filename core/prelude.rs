use std::any::Any;
use std::fmt::Debug;
use std::io;
use std::process::ExitStatus;

#[derive(Debug)]
pub enum Error {
    NotGitRepository,
    NotFound,
    Empty,
    ProcessError(ExitStatus),
    IoError(io::Error),
    ThreadError(Box<dyn Any + Send + 'static>),
}

impl PartialEq for Error {
    fn eq(&self, rhs: &Error) -> bool {
        let lhs = self;
        use Error::*;
        match (lhs, rhs) {
            (NotGitRepository, NotGitRepository) => true,
            (NotFound, NotFound) => true,
            (Empty, Empty) => true,
            (ProcessError(lhs), ProcessError(rhs)) => lhs == rhs,
            (IoError(lhs), IoError(rhs)) => lhs.kind() == rhs.kind(),
            (ThreadError(_), ThreadError(_)) => true,
            _ => false,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! from {
    ($from:ty, $variant:ident, $parent:ident) => {
        impl From<$from> for $parent {
            fn from(err: $from) -> $parent {
                $parent::$variant(err)
            }
        }
    };
}

from!(io::Error, IoError, Error);
from!(Box<dyn Any + Send + 'static>, ThreadError, Error);

impl Error {
    pub fn code(&self) -> u8 {
        match self {
            Error::ProcessError(e) => e.code().unwrap_or(1) as u8,
            Error::NotGitRepository => 128,
            _ => 1,
        }
    }
}

pub trait ToError<T> {
    fn to_err(self) -> Result<T>;
}
impl ToError<ExitStatus> for std::result::Result<ExitStatus, io::Error> {
    fn to_err(self) -> Result<ExitStatus> {
        match self {
            Err(e) => Err(Error::IoError(e)),
            Ok(v) if v.success() => Ok(v),
            Ok(v) => Err(Error::ProcessError(v)),
        }
    }
}

pub trait ToExitCode<T, E> {
    fn to_exit_code(self) -> std::process::ExitCode;
}

impl<T, E: Into<Error>> ToExitCode<T, E> for std::result::Result<T, E> {
    fn to_exit_code(self) -> std::process::ExitCode {
        use std::process::ExitCode;
        match self {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => ExitCode::from(e.into().code()),
        }
    }
}
