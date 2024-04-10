use std::collections::HashMap;
use std::process::Command;
use std::process::ExitCode;
use std::process::ExitStatus;

pub use crate::error::*;

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
