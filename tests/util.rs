use crate::test::Test;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::path::Path;
use std::process::Command;

pub trait TestCommands {
    /// Extracts stdout and stderr into strings for easying checking.
    fn stdout_string(&mut self) -> String;

    /// Loads commonly used parameters onto a `Command`
    fn load<I, S, P>(&mut self, args: I, cwd: P) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
        P: AsRef<Path>;
}

impl TestCommands for Command {
    fn stdout_string(&mut self) -> String {
        self.output().map_or("".to_string(), |v| {
            String::from_utf8_lossy(&v.stdout).to_string()
        })
    }

    fn load<I, S, P>(&mut self, args: I, cwd: P) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
        P: AsRef<Path>,
    {
        self.args(args).current_dir(cwd)
    }
}

/// Shell operations for a test
impl Test {
    /// Write text to a file
    pub fn write_file(&mut self, file: &str, contents: &str) -> &mut Self {
        if let Ok(mut f) = File::create(self.dir(file)) {
            use std::io::prelude::Write;
            f.write_all(contents.as_bytes()).ok();
        }
        self
    }

    /// Removes a file
    pub fn remove(&mut self, file: &str) -> &mut Self {
        fs::remove_file(self.dir(file)).ok();
        self
    }

    /// Renames a file
    pub fn rename(&mut self, file: &str, rename: &str) -> &mut Self {
        fs::rename(self.dir(file), self.dir(rename)).ok();
        self
    }
}
