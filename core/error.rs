use std::io::Error;
use std::process::{Child, ExitStatus};

pub enum GitnuError {
    ProcessError(ExitStatus),
    IoError(Error),
}

pub trait ToGitnuError<T> {
    fn gitnu_err(self) -> Result<T, GitnuError>;
}

impl ToGitnuError<Child> for Result<Child, Error> {
    fn gitnu_err(self) -> Result<Child, GitnuError> {
        self.map_err(GitnuError::IoError)
    }
}

impl ToGitnuError<ExitStatus> for Result<ExitStatus, Error> {
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
