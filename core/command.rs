use crate::{GitnuError, ToGitnuError};
use std::process::Command;

pub trait CommandOps {
    /// Runs the command gets exit code. Defaults to 1.
    ///
    /// Call this after parsing is complete and command is fully loaded
    /// with all the correct parameters.
    fn run(&mut self) -> Result<(), GitnuError>;
}

impl CommandOps for Command {
    fn run(&mut self) -> Result<(), GitnuError> {
        self.status().gitnu_err().map(|_| ())
    }
}
