use crate::utils::*;
use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

pub const TEST_DIR: &str = "gitnu-tests";

#[derive(PartialEq)]
pub struct Test {
    name: String,
    bin_path: PathBuf,
    test_dir: PathBuf,
    received: ShellOutputs,
    expected: ShellOutputs,
    asserted_once: bool,

    // to store run-specific git SHAs,
    // for tests which contain SHA outputs
    sha: String,
}

pub fn test(name: &str) -> Test {
    Test::new(name)
}

/// used for setting expected value of tests
/// "---" is used to prefix longer test outputs so that the string literal
/// can be flushed to the left.
fn set_expected(target: &mut String, val: &str, sha: &str) {
    if val.is_empty() {
        (*target).clear();
        return;
    }
    let v = val.split_once("---\n").map(|a| a.1).unwrap_or(val);
    *target = v.replace("[:SHA:]", sha);
}

impl Test {
    fn new(name: &str) -> Test {
        let name = String::from(name);
        let test_dir = env::temp_dir().join(TEST_DIR).join(&name);
        if test_dir.exists() {
            fs::remove_dir_all(&test_dir).ok();
        }
        let bin_path = env::current_exe()
            .unwrap()
            .parent()
            .expect("executable's directory")
            .to_path_buf()
            .join(format!("../gitnu{}", env::consts::EXE_SUFFIX));
        fs::create_dir_all(&test_dir).unwrap();
        Test {
            bin_path,
            sha: String::new(),
            asserted_once: false,
            received: ShellOutputs::default(),
            expected: ShellOutputs::default(),
            name,
            test_dir,
        }
    }

    /// get path to gitnu's debug binary build
    fn bin(&self) -> Command {
        Command::new(&self.bin_path)
    }

    /// Runs a `gitnu` command at a relative path from the test
    /// directory and populates `self.received` with output
    pub fn gitnu(&mut self, rel_path: &str, args: &str) -> &mut Self {
        let git_configs = [
            "user.name=bot",
            "user.email=bot@gitnu.co",
            "init.defaultBranch=main",
            "advice.statusHints=false",
        ];
        let mut cmd = self.bin();
        for config in git_configs {
            cmd.arg("-c").arg(config);
        }
        cmd.args(args.split(' '));
        cmd.current_dir(&self.test_dir.join(rel_path));
        self.received = cmd.outputs();
        self
    }

    /// Runs a shell command and populates `self.received` with output
    pub fn shell(&mut self, rel_path: &str, shell_cmd: &str) -> &mut Self {
        let args: Vec<&str> = shell_cmd.split(' ').collect();
        if args.len() == 0 {
            return self;
        }
        let mut cmd = Command::new(&args[0]);
        cmd.args(&args[1..]).current_dir(self.test_dir.join(rel_path));
        self.received = cmd.outputs();
        self
    }

    /// Gets the short SHA of the current commit during the test
    pub fn set_sha(&mut self) -> &mut Self {
        self.sha = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .current_dir(&self.test_dir)
            .outputs()
            .stdout
            .trim()
            .to_string();
        self
    }

    /// Write text to a file
    pub fn write_to_file(&mut self, rel_path: &str, text: &str) -> &mut Self {
        let file = self.test_dir.join(rel_path);
        let mut file = File::options().write(true).open(file).unwrap();
        use std::io::prelude::Write;
        file.write_all(text.as_bytes()).ok();
        self
    }

    /// removes a file
    pub fn remove(&mut self, rel_path: &str) -> &mut Self {
        let file = self.test_dir.join(rel_path);
        fs::remove_file(file).ok();
        self
    }

    /// renames a file
    pub fn rename(&mut self, curr: &str, next: &str) -> &mut Self {
        let file = self.test_dir.join(curr);
        if file.is_file() {
            fs::rename(file, self.test_dir.join(next))
                .expect("Unable to rename file");
        }
        self
    }

    /// Set expected stdout value.
    pub fn expect_stdout(&mut self, val: &str) -> &mut Self {
        set_expected(&mut self.expected.stdout, val, &self.sha);
        self
    }

    /// Set expected stderr value.
    pub fn expect_stderr(&mut self, val: &str) -> &mut Self {
        set_expected(&mut self.expected.stderr, val, &self.sha);
        self
    }

    pub fn assert(&mut self) -> &mut Self {
        self.asserted_once = true;
        assert_eq_pretty!(&self.expected.stdout, &self.received.stdout);
        assert_eq_pretty!(&self.expected.stderr, &self.received.stderr);
        self
    }
}

impl Drop for Test {
    /// asserts if hasn't, and then executes teardown
    fn drop(&mut self) {
        if !self.asserted_once {
            self.assert();
        }
        fs::remove_dir_all(&self.test_dir).ok();
    }
}
