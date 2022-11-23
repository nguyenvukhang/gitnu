use std::ffi::OsStr;
use std::process::Command;
use std::{env, fs, path::PathBuf};

use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Default)]
pub struct Test {
    name: String,
    test_dir: PathBuf,
    bin_path: PathBuf,
}

const TEST_DIR: &str = "gitnu-tests";

trait Run {
    fn run(&mut self);
}

impl Run for Command {
    fn run(&mut self) {
        self.output().ok();
    }
}

impl Test {
    pub fn new() -> Test {
        let mut test = Test::default();
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        test.name = id.to_string();
        test.test_dir = env::temp_dir().join(TEST_DIR).join(&test.name);
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

    fn cmd<S: AsRef<OsStr>>(&self, cmd: S) -> Command {
        let mut cmd = Command::new(cmd);
        cmd.current_dir(&self.test_dir);
        cmd
    }

    /// creates `file_count` number of files in the test directory
    /// and initializes it as a git repository
    pub fn setup(&self, file_count: usize) {
        self.cmd("git").arg("init").run();
        let mut touch = self.cmd("touch");
        for i in 0..file_count {
            touch.arg(format!("file_{}", i));
        }
        touch.run();
    }

    pub fn run_gitnu(&self) {
        self.cmd(&self.bin_path).arg("status").run();
    }

    pub fn run_git(&self) {
        self.cmd("git").arg("status").run();
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.test_dir).ok();
    }
}
