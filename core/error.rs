use std::io;
use std::process::ExitStatus;

pub enum GitnuError {
    ProcessError(ExitStatus),
    IoError(io::Error),
}

pub trait ToGitnuError<T> {
    fn gitnu_err(self) -> Result<T, GitnuError>;
}

impl From<io::Error> for GitnuError {
    fn from(err: io::Error) -> GitnuError {
        GitnuError::IoError(err)
    }
}

impl ToGitnuError<ExitStatus> for Result<ExitStatus, io::Error> {
    fn gitnu_err(self) -> Result<ExitStatus, GitnuError> {
        match self {
            Err(e) => Err(GitnuError::IoError(e)),
            Ok(v) if v.success() => Ok(v),
            Ok(v) => Err(GitnuError::ProcessError(v)),
        }
    }
}

impl GitnuError {
    pub fn code(&self) -> u8 {
        match self {
            GitnuError::ProcessError(e) => e.code().unwrap_or(1) as u8,
            _ => 1,
        }
    }
}
