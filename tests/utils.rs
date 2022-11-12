use crate::result::*;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::Output;

pub const TMP_DIR: &str = "/tmp/gitnu_rust";

pub trait SafeJoin {
    fn safe_join(&self, relative_path: &str) -> Result<PathBuf>;
}

impl SafeJoin for PathBuf {
    fn safe_join(&self, relative_path: &str) -> Result<PathBuf> {
        let p = self.join(relative_path);
        p.is_dir().then_some(p).ok_or("bad relative path".to_string())
    }
}

pub fn pretty_assert<T: PartialEq + Display>(expected: &T, received: &T) {
    if !(*received == *expected) {
        eprintln!("<<<<<<< EXPECTED\n{}", expected);
        eprintln!("================\n{}", received);
        eprintln!(">>>>>>> RECEVIED");
        panic!("Test failed.");
    }
}

pub trait ShellString {
    fn stdout_string(&self) -> String;
    fn stderr_string(&self) -> String;
}

impl ShellString for Output {
    fn stdout_string(&self) -> String {
        String::from(String::from_utf8_lossy(&self.stdout))
    }
    fn stderr_string(&self) -> String {
        String::from(String::from_utf8_lossy(&self.stderr))
    }
}
