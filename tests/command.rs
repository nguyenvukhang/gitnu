use crate::result::*;
use crate::utils::SafeJoin;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;

pub trait CommandBuilder {
    /// Builds a command from a bin path and some arguments.
    fn set_args<S: AsRef<OsStr>>(
        self,
        args: impl Iterator<Item = S>,
    ) -> Command;

    /// Mutates self and sets current directory to base + relative.
    /// Fails if the resulting path doesn't exist.
    fn set_dir<P: AsRef<Path>>(self, base: &PathBuf, rel: P)
        -> Result<Command>;
}

impl CommandBuilder for Command {
    fn set_args<S: AsRef<OsStr>>(
        mut self,
        args: impl Iterator<Item = S>,
    ) -> Command {
        self.args(args);
        self
    }

    fn set_dir<P: AsRef<Path>>(
        mut self,
        base: &PathBuf,
        rel: P,
    ) -> Result<Command> {
        self.current_dir(base.safe_join(rel).unwrap());
        Ok(self)
    }
}
