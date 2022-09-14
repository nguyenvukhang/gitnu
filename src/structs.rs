use crate::actions;
use crate::shell;
use log::info;
use serde_json::{from_str, to_vec};
use std::fs::{read_to_string, write};
use std::io::Error;
use std::path::PathBuf;
use std::process::Command;

pub enum OpType {
    Read,
    Status,
    Bypass,
    Xargs,
}

pub struct Gitnu {
    pub cmd: Command,
    pub op: OpType,
    json_file: PathBuf,
    git_dir: PathBuf,
    files: Vec<String>,
    git_root: PathBuf,
}

/// get the path to the git repository/worktree
/// this will be where the cache is stored
fn get_cache_dir(git_dir: &String) -> PathBuf {
    let mut git = Command::new("git");
    let cmd = git.arg("-C").arg(git_dir).args([
        "rev-parse",
        "--path-format=absolute",
        "--git-dir",
    ]);
    let res = match shell::get_stdout(cmd) {
        Ok(v) => PathBuf::from(v),
        _ => PathBuf::from("/dev/null"),
    };
    res
}

fn get_git_root(git_dir: &String) -> PathBuf {
    let mut git = Command::new("git");
    let cmd = git.arg("-C").arg(git_dir).args([
        "rev-parse",
        "--path-format=absolute",
        "--show-toplevel",
    ]);
    let mut path = PathBuf::from(".");
    let res = match shell::get_stdout(cmd) {
        Ok(v) => PathBuf::from(v),
        _ => PathBuf::from("/dev/null"),
    };
    path.push(res);
    path
}

impl Gitnu {
    /// takes in a list of args to the left of the command
    /// (excluding the command itself)
    pub fn new(op: OpType, git_dir: String) -> Gitnu {
        // get path to git root
        Gitnu {
            json_file: get_cache_dir(&git_dir).join("gitnu.json"),
            cmd: Command::new(""),
            files: Vec::new(),
            git_root: get_git_root(&git_dir),
            git_dir: PathBuf::from(&git_dir),
            op,
        }
    }

    /// load runtime buffer with files in order
    pub fn load_files(&mut self) {
        let path = PathBuf::from(&self.git_dir);
        actions::load_files(self, &path);
    }

    /// add a file to runtime buffer
    pub fn add_file(&mut self, file: String) {
        self.files.push(file);
    }

    /// write data to json cache
    pub fn write_json(&self) {
        info!("writing to json: {:#?}", self.json_file);
        let serialized = to_vec(&self.files).expect("Unable to serialize");
        write(&self.json_file, serialized)
            .expect("Unable to write to gitnu.json");
    }

    /// read from json cache
    pub fn read_json(&mut self) -> Result<(), Error> {
        info!("reading from json: {:#?}", self.json_file);
        match read_to_string(&self.json_file) {
            Ok(v) => {
                let parsed: Vec<String> =
                    from_str(&v).expect("Unable to parse gitnu.json");
                // bump the zeroth element to deal with indexing
                self.files = Vec::from([String::from("$zero")]);
                Ok(self.files.extend(parsed))
            }
            Err(e) => Err(e),
        }
    }

    /// get a file by index
    pub fn get_file(&self, index: usize) -> Result<String, ()> {
        if index >= self.files.len() {
            return Err(());
        };
        let f = self.git_root.join(&self.files[index]);
        Ok(f.into_os_string().into_string().unwrap())
    }
}
