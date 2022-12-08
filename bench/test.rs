use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{env, fs, path::PathBuf};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

/// A test with a unique id, a dedicated temporary directory,
/// and a path to the gitnu binary
#[derive(Default)]
pub struct Test {
    name: String,
    test_dir: PathBuf,
    bin_path: PathBuf,
    ideal_path: PathBuf,
}

const TEST_DIR: &str = "gitnu-tests";

fn bin(exe: &str) -> PathBuf {
    env::current_exe()
        .unwrap()
        .parent()
        .expect("executable's directory")
        .to_path_buf()
        .join(format!("../{}{}", exe, env::consts::EXE_SUFFIX))
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
        test.bin_path = bin("gitnu");
        test.ideal_path = bin("gitnu-ideal");
        test
    }

    pub fn dir(&self) -> &PathBuf {
        &self.test_dir
    }

    pub fn file(&self, name: &str) -> fs::File {
        fs::File::create(&self.test_dir.join(name)).unwrap()
    }

    pub fn cmd(&self, cmd: &str) -> Command {
        let mut cmd = match cmd {
            "gitnu" => Command::new(&self.bin_path),
            "ideal" => Command::new(&self.ideal_path),
            v => Command::new(v),
        };
        cmd.current_dir(&self.test_dir);
        cmd
    }

    /// creates `file_count` number of files in the test directory
    /// and initializes it as a git repository
    pub fn setup(&self, file_count: u32) {
        self.cmd("git").arg("init").output().ok();
        let mut touch = self.cmd("touch");
        for i in 0..file_count {
            touch.arg(format!("file_{}", i));
        }
        touch.output().ok();
    }
}

impl Drop for Test {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.test_dir).ok();
    }
}
