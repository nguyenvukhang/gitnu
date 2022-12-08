use crate::util::TestCommands;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub const TEST_DIR: &str = "gitnu-tests";

#[derive(Default)]
pub struct Test {
    name: String,
    bin_path: PathBuf,
    test_dir: PathBuf,

    /// [normal, short]
    checked: [bool; 2],

    /// to store run-specific git SHAs,
    /// for tests which contain SHA outputs
    sha: String,
}

impl Test {
    /// Creates a command pointing to gitnu's debug binary build
    fn bin(&self) -> Command {
        let mut cmd = Command::new(&self.bin_path);
        let git_configs = [
            "user.name=bot",
            "user.email=bot@gitnu.co",
            "init.defaultBranch=main",
            "advice.statusHints=false",
            "color.ui=always",
        ];
        for config in git_configs {
            cmd.arg("-c").arg(config);
        }
        cmd
    }

    /// Gets the path to the root of the test directory.
    pub fn dir(&self, path: &str) -> PathBuf {
        self.test_dir.join(path)
    }

    fn command(&self, path: &str, cmd: &str) -> Option<Command> {
        assert_ne!(cmd, "");
        let mut args = cmd.split(' ');
        let mut cmd = match args.next() {
            Some("gitnu") => self.bin(),
            Some(v) => Command::new(v),
            _ => return None,
        };
        cmd.load(args, self.dir(path));
        Some(cmd)
    }

    /// Gets the stdout of a shell command ran at path `path` as a `String`
    fn stdout(&self, cmd: &str, path: &str) -> String {
        assert_ne!(cmd, "");
        match self.command(path, cmd) {
            Some(v) => v,
            None => return String::new(),
        }
        .stdout_string()
        .replace("\x1b[31m", "")
        .replace("\x1b[32m", "")
        .replace("\x1b[m", "")
    }

    /// Assert a shell command's output when ran at a given `path`.
    pub fn assert(&mut self, path: &str, cmd: &str, expected: &str) {
        let expected = expected.replace("[:SHA:]", &self.sha);
        assert_eq_pretty!(self.stdout(cmd, path), expected);
    }
}

impl Test {
    /// Creates a new test, along with a unique directory to test it in
    pub fn new(name: &str) -> Test {
        let mut test = Test::default();
        test.name = name.to_string();
        test.test_dir = env::temp_dir().join(TEST_DIR).join(&name);
        if test.test_dir.exists() {
            fs::remove_dir_all(&test.test_dir).ok();
        }
        fs::create_dir_all(&test.test_dir).unwrap();
        test.bin_path = env::current_exe().unwrap();
        test.bin_path.pop();
        test.bin_path.pop();
        test.bin_path.push(format!("gitnu{}", env::consts::EXE_SUFFIX));
        test
    }

    /// Gets the path to the test dir
    pub fn get_test_dir(&self) -> PathBuf {
        let p = PathBuf::from(&self.test_dir);
        std::fs::canonicalize(&p).unwrap_or(p)
    }

    /// Runs a `gitnu` command at a relative path from the test directory
    pub fn gitnu(&mut self, path: &str, args: &str) -> &mut Self {
        self.bin().load(args.split(' '), self.dir(path)).output().ok();
        self
    }

    /// Runs a shell command and populates `self.received` with output
    pub fn shell(&mut self, path: &str, shell_cmd: &str) -> &mut Self {
        assert_ne!(shell_cmd, "");
        match self.command(path, shell_cmd) {
            Some(mut v) => v.output().ok(),
            _ => None,
        };
        self
    }

    /// Gets the short SHA of the current commit during the test
    pub fn set_sha(&mut self) -> &mut Self {
        self.sha = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .current_dir(&self.test_dir)
            .stdout_string()
            .trim()
            .to_string();
        self
    }

    /// Mocks a cache file with a list of filenames
    pub fn mock_cache(&self, files: Vec<&str>) -> String {
        let test_dir = self.get_test_dir();
        files
            .iter()
            .map(|file| test_dir.join(file))
            .map(|path| path.to_string_lossy().to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    /// Make assertion about a command's exit code
    pub fn assert_exit_code(&mut self, path: &str, cmd: &str, code: i32) {
        assert_ne!(cmd, "");
        let received = self
            .command(path, cmd)
            .unwrap()
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .status()
            .ok()
            .and_then(|p| p.code());
        assert_eq_pretty!(received.unwrap_or(-1), code);
    }

    /// Tells the test suite to not hound this test case for checks
    pub fn mark_as_checked(&mut self) {
        self.checked = [true, true];
    }

    /// Marks this `gitnu status` type as done
    pub fn mark_status(&mut self, is_normal: bool) {
        self.checked[0] |= is_normal;
        self.checked[1] |= !is_normal;
    }
}

impl Drop for Test {
    /// asserts if hasn't, and then executes teardown
    fn drop(&mut self) {
        if !std::thread::panicking() {
            match self.checked {
                [false, _] => panic!("`gitnu status` not checked."),
                [_, false] => panic!("`gitnu status --short` not checked."),
                _ => (),
            }
        }
        fs::remove_dir_all(&self.test_dir).ok();
    }
}
