use std::path::PathBuf;
use std::process::Command;

pub trait CommandOps {
    /// Runs the command gets exit code. Defaults to 1.
    ///
    /// Call this after parsing is complete and command is fully loaded
    /// with all the correct parameters.
    fn run(&mut self) -> i32;

    /// Get stduout as a string
    fn stdout_string(&mut self) -> String;

    /// Get stduout as a pathbuf
    fn stdout_pathbuf(&mut self) -> Option<PathBuf>;
}

impl CommandOps for Command {
    fn run(&mut self) -> i32 {
        self.status().ok().and_then(|v| v.code()).unwrap_or(1)
    }

    fn stdout_string(&mut self) -> String {
        self.output().map_or("".to_string(), |v| {
            String::from_utf8_lossy(&v.stdout).to_string()
        })
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
