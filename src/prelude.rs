use std::collections::HashMap;
use std::process::{ExitCode, ExitStatus};

pub(crate) use crate::cache::Cache;
pub(crate) use crate::error::*;
pub(crate) use crate::git_cmd::*;
pub(crate) use crate::pathdiff;
pub(crate) use crate::traits::*;

pub(crate) const MAX_CACHE_SIZE: usize = 20;
pub(crate) const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) const CACHE_FILE_NAME: &str = "gitnu.txt";

pub type Result<T> = std::result::Result<T, Error>;
pub type Aliases = HashMap<String, String>;

pub trait ToExitCode {
    fn exitcode(self) -> ExitCode;
}

impl ToExitCode for ExitStatus {
    fn exitcode(self) -> ExitCode {
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
