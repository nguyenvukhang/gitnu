use std::io;
use std::process::ExitStatus;

#[derive(Debug)]
pub enum Error {
    NotGitRepository,
    NotFound,
    Empty,
    ProcessError(ExitStatus),
    IoError(io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

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

#[macro_export]
macro_rules! assert_eq {
    ($received:expr, $expected:expr) => {
        if $received != $expected {
            panic!(
                "
\x1b[0;31mReceived:\x1b[0m
{received:?}
─────────────────────────────────────────────────────────────
\x1b[0;32mExpected:\x1b[0m
{expected:?}
",
                received = $received,
                expected = $expected
            );
        }
    };
}
