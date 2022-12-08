use crate::{GitnuError, ToGitnuError};
use std::path::PathBuf;
use std::process::Command;

pub trait CommandOps {
    /// Runs the command gets exit code. Defaults to 1.
    ///
    /// Call this after parsing is complete and command is fully loaded
    /// with all the correct parameters.
    fn run(&mut self) -> Result<(), GitnuError>;

    /// Get stduout as a pathbuf
    fn stdout_pathbuf(&mut self) -> Option<PathBuf>;
}

impl CommandOps for Command {
    fn run(&mut self) -> Result<(), GitnuError> {
        self.status().gitnu_err().map(|_| ())
    }

    fn stdout_pathbuf(&mut self) -> Option<PathBuf> {
        let output = self.output().ok()?;
        if !output.status.success() {
            return None;
        }
        match String::from_utf8_lossy(&output.stdout).trim() {
            v if v.is_empty() => None,
            v => Some(PathBuf::from(v)),
        }
    }
}
