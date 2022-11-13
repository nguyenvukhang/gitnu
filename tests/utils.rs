use std::process::Command;

/// An stdout-stderr pair to capture full shell output for easy checking.
#[derive(PartialEq, Default)]
pub struct ShellOutputs {
    pub stdout: String,
    pub stderr: String,
}

pub trait ShellString {
    /// Extracts stdout and stderr into strings for easying checking.
    fn outputs(&mut self) -> ShellOutputs;
}

fn stringify(v: &[u8]) -> String {
    String::from_utf8_lossy(v).parse().unwrap_or("".to_string())
}

impl ShellString for Command {
    fn outputs(&mut self) -> ShellOutputs {
        self.output()
            .map(|v| ShellOutputs {
                stdout: stringify(&v.stdout),
                stderr: stringify(&v.stderr),
            })
            .unwrap_or_default()
    }
}

pub type Result<T> = std::result::Result<T, String>;

pub trait StringError<T, E> {
    fn serr(self, err_msg: &str) -> Result<T>;
    fn clear(self) -> std::result::Result<(), E>;
}

impl<T, E> StringError<T, E> for std::result::Result<T, E> {
    fn serr(self, err_msg: &str) -> Result<T> {
        self.map_err(|_| err_msg.to_string())
    }
    fn clear(self) -> std::result::Result<(), E> {
        return self.map(|_| ());
    }
}
