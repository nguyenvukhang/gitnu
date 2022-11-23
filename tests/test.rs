use std::env;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;

pub const TEST_DIR: &str = "gitnu-tests";

trait Stringify {
    /// Extracts stdout and stderr into strings for easying checking.
    fn stdout_string(&mut self) -> String;
}

impl Stringify for Command {
    fn stdout_string(&mut self) -> String {
        self.output().map_or("".to_string(), |v| {
            String::from_utf8_lossy(&v.stdout).to_string()
        })
    }
}

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
    /// get path to gitnu's debug binary build
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

    fn dir(&self, path: &str) -> PathBuf {
        self.test_dir.join(path)
    }

    fn stdout(&self, cmd_str: &str, path: &str) -> String {
        assert_ne!(cmd_str, "");
        let mut args = cmd_str.split(' ');
        match args.next().unwrap() {
            "gitnu" => self.bin(),
            v => Command::new(v),
        }
        .args(args)
        .current_dir(&self.dir(path))
        .stdout_string()
        .replace("\x1b[31m", "")
        .replace("\x1b[32m", "")
        .replace("\x1b[m", "")
    }

    fn get_expected(&self, s: &str) -> String {
        if s.is_empty() {
            return "".to_string();
        }
        s.split_once("---\n").map_or(s, |a| a.1).replace("[:SHA:]", &self.sha)
    }

    fn assert(&mut self, path: &str, expect: &str, cmd: &str, mask: [bool; 2]) {
        self.checked[0] |= mask[0];
        self.checked[1] |= mask[1];
        assert_eq_pretty!(self.get_expected(expect), self.stdout(cmd, path));
    }
}

pub trait TestInterface {
    /// Creates a new test, along with a unique directory to test it in
    fn new(name: &str) -> Test;
    /// Runs a `gitnu` command at a relative path from the test directory
    fn gitnu(&mut self, path: &str, args: &str) -> &mut Self;
    /// Runs a shell command and populates `self.received` with output
    fn shell(&mut self, path: &str, shell_cmd: &str) -> &mut Self;
    /// Gets the short SHA of the current commit during the test
    fn set_sha(&mut self) -> &mut Self;
    /// Gets the short SHA of the current commit during the test
    fn extract_stdout(&mut self, cmd: &str, dst: &mut String) -> &mut Self;
    /// Gets the path to the test dir
    fn get_test_dir(&self) -> PathBuf;
    /// Write text to a file
    fn write_file(&mut self, path: &str, text: &str) -> &mut Self;
    /// Removes a file
    fn remove(&mut self, path: &str) -> &mut Self;
    /// Renames a file
    fn rename(&mut self, curr: &str, next: &str) -> &mut Self;
    /// make assertion about a general command
    fn assert_general(&mut self, path: &str, expect: &str, cmd: &str);
    /// make assertion on `gitnu status`
    fn assert_normal(&mut self, path: &str, expect: &str);
    /// make assertion on `gitnu status --short`
    fn assert_short(&mut self, path: &str, expect: &str);
}

impl TestInterface for Test {
    fn new(name: &str) -> Test {
        let mut test = Test::default();
        test.name = name.to_string();
        test.test_dir = env::temp_dir().join(TEST_DIR).join(&name);
        if test.test_dir.exists() {
            fs::remove_dir_all(&test.test_dir).ok();
        }
        fs::create_dir_all(&test.test_dir).unwrap();
        test.bin_path = env::current_exe()
            .unwrap()
            .parent()
            .expect("executable's directory")
            .to_path_buf()
            .join(format!("../gitnu{}", env::consts::EXE_SUFFIX));
        test
    }

    fn get_test_dir(&self) -> PathBuf {
        let p = PathBuf::from(&self.test_dir);
        std::fs::canonicalize(&p).unwrap_or(p)
    }

    fn gitnu(&mut self, path: &str, args: &str) -> &mut Self {
        self.bin()
            .current_dir(&self.dir(path))
            .args(args.split(' '))
            .output()
            .ok();
        self
    }

    fn extract_stdout(&mut self, cmd: &str, dst: &mut String) -> &mut Self {
        assert_ne!(cmd, "");
        let mut args = cmd.split(' ');
        *dst = Command::new(args.next().unwrap())
            .args(args)
            .current_dir(&self.test_dir)
            .stdout_string();
        self
    }

    fn shell(&mut self, path: &str, shell_cmd: &str) -> &mut Self {
        assert_ne!(shell_cmd, "");
        let mut args = shell_cmd.split(' ');
        Command::new(args.next().unwrap())
            .args(args)
            .current_dir(self.dir(path))
            .output()
            .ok();
        self
    }

    fn set_sha(&mut self) -> &mut Self {
        self.sha = Command::new("git")
            .args(["rev-parse", "--short", "HEAD"])
            .current_dir(&self.test_dir)
            .stdout_string()
            .trim()
            .to_string();
        self
    }

    fn write_file(&mut self, path: &str, text: &str) -> &mut Self {
        use std::io::prelude::Write;
        if let Ok(mut f) = File::create(self.dir(path)) {
            f.write_all(text.as_bytes()).ok();
        }
        self
    }

    fn remove(&mut self, path: &str) -> &mut Self {
        fs::remove_file(self.dir(path)).ok();
        self
    }

    fn rename(&mut self, curr: &str, next: &str) -> &mut Self {
        fs::rename(self.dir(curr), self.dir(next)).ok();
        self
    }

    fn assert_general(&mut self, path: &str, expect: &str, cmd: &str) {
        self.assert(path, expect, cmd, [true, true]);
    }

    fn assert_normal(&mut self, path: &str, expect: &str) {
        self.assert(path, expect, "gitnu status", [true, false]);
    }

    fn assert_short(&mut self, path: &str, expect: &str) {
        self.assert(path, expect, "gitnu status --short", [false, true]);
    }
}

impl Drop for Test {
    /// asserts if hasn't, and then executes teardown
    fn drop(&mut self) {
        if !std::thread::panicking() {
            if !self.checked[0] {
                panic!("`gitnu status` output not checked.")
            }
            if !self.checked[1] {
                panic!("`gitnu status --short` output not checked.")
            }
        }
        fs::remove_dir_all(&self.test_dir).ok();
    }
}
