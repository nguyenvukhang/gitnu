use crate::command::CommandBuilder;
use crate::result::*;
use crate::utils::*;
use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

#[derive(PartialEq)]
pub struct Test {
    called_once: bool,
    name: String, // always call with module_path!()
    test_dir: PathBuf,
    bin: PathBuf,
    received: ShellOutputs,
    expected: ShellOutputs,
    sha: String,
}

pub fn test(name: &str) -> Test {
    Test::new(name)
}

fn test_dir() -> String {
    env::var("GITNU_TEST_DIR").unwrap_or("/tmp/gitnu_rust".into())
}

/// get path to gitnu's debug binary build
fn bin_path() -> Result<PathBuf> {
    let prj_dir = env::var("CARGO_MANIFEST_DIR")
        .serr("Unable to use env to locate gitnu's Cargo project dir.")?;
    let bin = PathBuf::from(prj_dir).join("target/debug/gitnu");
    if !bin.is_file() {
        return err("gitnu debug binary not found.");
    }
    Ok(bin)
}

/// used for setting expected value of tests
/// "---" is used to prefix longer test outputs so that the string literal
/// can be flushed to the left.
fn set_expected(target: &mut String, val: &str, sha: &str) {
    if val.is_empty() {
        (*target).clear();
        return;
    }
    let v = val.split_once("---\n").map(|(a, b)| (a, String::from(b)));
    let v = v.unwrap_or(("", val.to_string())).1;
    *target = v.replace("[:SHA:]", sha);
}

/// test file layout:
/// /TMP_DIR (defaults to /tmp/gitnu)
///   - [name]/
///     - all test files/temporary repos/...
///
/// throw all errors right here if init fails, hence the unwraps
impl Test {
    fn new(name: &str) -> Test {
        let name = String::from(name);
        let test_dir = PathBuf::from(test_dir()).join(&name);
        if !test_dir.is_absolute() {
            panic!("Use an absolute path for tests");
        }
        fs::create_dir_all(&test_dir).unwrap();
        env::set_current_dir(&test_dir).unwrap();
        Test {
            sha: String::new(),
            called_once: false,
            bin: bin_path().unwrap(),
            received: ShellOutputs::default(),
            expected: ShellOutputs::default(),
            name,
            test_dir,
        }
    }

    /// simulates a `gitnu` binary run at a specific relative path
    /// from the test directory. Also saves stdout to `self.recevied`
    pub fn gitnu(&mut self, rel_path: &str, args: &str) -> &mut Self {
        // test-specific configs to replicate exact expected outputs
        let git_configs = [
            "user.name=bot",
            "user.email=bot@gitnu.co",
            "init.defaultBranch=main",
            "advice.statusHints=false",
        ];
        let mut cmd = Command::new(&self.bin);
        for config in git_configs {
            cmd.arg("-c").arg(config);
        }
        self.received = cmd
            .set_args(args.split(' '))
            .set_dir(&self.test_dir, rel_path)
            .unwrap()
            .output()
            .unwrap()
            .outputs();
        self
    }

    /// runs a shell command
    pub fn shell(&mut self, rel_path: &str, shell_cmd: &str) -> &mut Self {
        let cwd = match self.test_dir.safe_join(rel_path) {
            Ok(v) => v,
            Err(_) => return self,
        };
        let args: Vec<&str> = shell_cmd.split(' ').collect();
        self.received = Command::new(&args[0])
            .args(&args[1..])
            .current_dir(cwd)
            .output()
            .unwrap()
            .outputs();
        self
    }

    pub fn set_sha(&mut self) -> &mut Self {
        self.gitnu("", "rev-parse --short HEAD");
        std::mem::swap(&mut self.sha, &mut self.received.stdout);
        self.sha = String::from(self.sha.trim());
        self
    }

    /// append text to a file
    pub fn append_to_file(&mut self, file_path: &str, text: &str) -> &mut Self {
        let file_path = self.test_dir.safe_join(file_path).unwrap();
        let mut f = File::options().append(true).open(&file_path).unwrap();
        use std::io::prelude::Write;
        f.write_all(text.as_bytes()).unwrap();
        self
    }

    /// append text to a file
    pub fn write_to_file(&mut self, file_path: &str, text: &str) -> &mut Self {
        let file_path = self.test_dir.safe_join(file_path).unwrap();
        let mut f = File::options().write(true).open(&file_path).unwrap();
        use std::io::prelude::Write;
        f.write_all(text.as_bytes()).unwrap();
        self
    }

    /// removes a file/directory recursively
    pub fn remove(&mut self, file_path: &str) -> &mut Self {
        let f = self.test_dir.safe_join(file_path).unwrap();
        if f.is_file() {
            fs::remove_file(f).ok();
        } else if f.is_dir() {
            fs::remove_dir_all(f).ok();
        }
        self
    }

    /// renames a file
    pub fn rename(&mut self, curr: &str, next: &str) -> &mut Self {
        let file_path = self.test_dir.safe_join(curr).unwrap();
        if file_path.is_file() {
            fs::rename(file_path, self.test_dir.join(next))
                .expect("Unable to rename file");
        }
        self
    }

    pub fn expect_stdout(&mut self, val: &str) -> &mut Self {
        set_expected(&mut self.expected.stdout, val, &self.sha);
        self
    }

    pub fn expect_stderr(&mut self, val: &str) -> &mut Self {
        set_expected(&mut self.expected.stderr, val, &self.sha);
        self
    }

    pub fn assert(&mut self) -> &mut Self {
        self.called_once = true;
        assert_eq_pretty!(&self.expected.stdout, &self.received.stdout);
        assert_eq_pretty!(&self.expected.stderr, &self.received.stderr);
        self
    }
}

impl Drop for Test {
    /// asserts that self.received and self.expected are equal,
    /// and then executes teardown
    fn drop(&mut self) {
        if !self.called_once {
            self.assert();
        }
        fs::remove_dir_all(&self.test_dir).ok();
    }
}
