use crate::result::*;
use crate::utils::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

#[derive(Debug, PartialEq, Default)]
struct Out2 {
    stdout: String,
    stderr: String,
}

impl Out2 {
    fn new(output: Output) -> Self {
        Self { stdout: output.stdout_string(), stderr: output.stderr_string() }
    }
}

#[derive(Debug, PartialEq)]
pub struct Test {
    name: String, // always call with module_path!()
    test_dir: PathBuf,
    bin: PathBuf,
    received: Out2,
    expected: Out2,
    previous: Out2,
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
fn set_expected(target: &mut String, val: &str) {
    *target =
        String::from(val.split_once("---\n").expect("Unlikely test format").1);
}

/// private functions
impl Test {
    fn gitnu_cmd<'a, T: Iterator<Item = &'a str>>(
        &self,
        path: &str,
        args: T,
    ) -> Result<Command> {
        let mut cmd = Command::new(&self.bin);
        let cwd = self.test_dir.safe_join(path)?;
        cmd.args(args).current_dir(cwd);
        Ok(cmd)
    }

    /// removes temporary files
    fn teardown(&self) {
        fs::remove_dir_all(&self.test_dir).ok();
    }
}

pub fn test(module_path: &str, name: &str) -> Test {
    Test::new(module_path, name)
}

/// test file layout:
/// /TMP_DIR (defaults to /tmp/gitnu)
///   - [name]/
///     - all test files/temporary repos/...
///
/// throw all errors right here if init fails, hence the unwraps
impl Test {
    fn new(module_path: &str, name: &str) -> Test {
        let name = format!("{}::{}", module_path, name);
        let test_dir = PathBuf::from(TMP_DIR).join(&name);
        if !test_dir.is_absolute() {
            panic!("Use an absolute path for tests");
        }
        fs::create_dir_all(&test_dir).unwrap();
        env::set_current_dir(&test_dir).unwrap();
        Test {
            bin: bin_path().unwrap(),
            received: Out2::default(),
            expected: Out2::default(),
            previous: Out2::default(),
            name,
            test_dir,
        }
    }

    /// simulates a `gitnu` binary run at a specific relative path
    /// from the test directory. Also saves stdout to `self.recevied`
    pub fn gitnu(&mut self, rel_path: &str, args: &str) -> &mut Self {
        let mut cmd = match self.gitnu_cmd(rel_path, args.split(' ')) {
            Err(_) => return self,
            Ok(v) => v,
        };
        // let mut cmd = self.gitnu_cmd(rel_path, args.split(' '));
        let out = cmd.output().unwrap();
        self.received = Out2::new(out);
        self
    }

    /// runs a shell command
    pub fn shell(&mut self, rel_path: &str, shell_cmd: &str) -> &mut Self {
        let cwd = match self.test_dir.safe_join(rel_path) {
            Ok(v) => v,
            Err(_) => return self,
        };
        let args: Vec<&str> = shell_cmd.split(' ').collect();
        let output = Command::new(&args[0])
            .args(&args[1..])
            .current_dir(cwd)
            .output()
            .unwrap();
        self.previous = Out2::new(output);
        self
    }

    pub fn expect_stdout(&mut self, val: &str) -> &mut Self {
        match val.is_empty() {
            true => self.expected.stdout.clear(),
            false => set_expected(&mut self.expected.stdout, val),
        }
        self
    }

    pub fn expect_stderr(&mut self, val: &str) -> &mut Self {
        match val.is_empty() {
            true => self.expected.stderr.clear(),
            false => set_expected(&mut self.expected.stderr, val),
        }
        self
    }

    pub fn assert(&self) {
        pretty_assert(&self.expected.stdout, &self.received.stdout);
        pretty_assert(&self.expected.stderr, &self.received.stderr);
    }
}

impl Drop for Test {
    /// asserts that self.received and self.expected are equal,
    /// and then executes teardown
    fn drop(&mut self) {
        self.assert();
        self.teardown();
    }
}
