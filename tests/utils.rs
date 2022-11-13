use crate::result::*;
use std::path::{Path, PathBuf};
use std::process::Output;

pub trait SafeJoin {
    /// Returns Err if the resulting path does not exist
    fn safe_join<P: AsRef<Path>>(&self, relative_path: P) -> Result<PathBuf>;
}

impl SafeJoin for PathBuf {
    fn safe_join<P: AsRef<Path>>(&self, relative_path: P) -> Result<PathBuf> {
        let p = self.join(relative_path);
        match p.is_dir() || p.is_file() {
            true => Ok(p),
            false => err(&format!("Bad relative path: {}", p.display())),
        }
    }
}

/// stdout-stderr pair to capture full shell output for easy checking.
#[derive(PartialEq, Default)]
pub struct ShellOutputs {
    pub stdout: String,
    pub stderr: String,
}

pub trait ShellString {
    /// extracts stdout and stderr into strings for easying checking.
    fn outputs(&self) -> ShellOutputs;
    fn stdout_string(&self) -> String;
    fn stderr_string(&self) -> String;
}

fn stringify(v: &[u8]) -> String {
    String::from(String::from_utf8_lossy(v))
}

impl ShellString for Output {
    fn stdout_string(&self) -> String {
        stringify(&self.stdout)
    }
    fn stderr_string(&self) -> String {
        stringify(&self.stderr)
    }
    fn outputs(&self) -> ShellOutputs {
        ShellOutputs {
            stdout: stringify(&self.stdout),
            stderr: stringify(&self.stderr),
        }
    }
}
