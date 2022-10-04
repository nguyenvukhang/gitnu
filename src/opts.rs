use std::io::{Error, ErrorKind::NotFound};
use std::path::PathBuf;
use std::process::Command;

pub fn get_cmd(opts: &Opts) -> Option<Command> {
    let cmd = match opts.op {
        OpType::Xargs => opts.xargs_cmd.as_ref()?,
        _ => "git",
    };
    let mut cmd = Command::new(cmd);
    cmd.current_dir(&opts.cwd);
    Some(cmd)
}

pub fn run(cmd: Option<Command>, args: Vec<PathBuf>) -> Result<(), Error> {
    let mut cmd = cmd.ok_or(Error::new(NotFound, "Command not found"))?;
    cmd.args(args).spawn()?.wait().map(|_| ())
}

pub const CACHE_FILE: &str = "gitnu.txt";

#[derive(Debug, PartialEq, Clone)]
pub enum OpType {
    Status, // gitnu status
    Read,   // gitnu add 2-4
    Bypass, // gitnu log --oneline
    Xargs,  // gitnu -c nvim 2
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum StatusFmt {
    Normal, // gitnu status
    Short,  // gitnu status --short || gitnu status --porcelain
}

#[derive(Debug, PartialEq, Clone)]
pub struct Opts {
    pub xargs_cmd: Option<String>,
    pub op: OpType,
    pub status_fmt: StatusFmt,
    pub git_root: Option<PathBuf>,
    pub cwd: PathBuf,
}

impl Opts {
    /// Initialize with empty options.
    pub fn new() -> Self {
        Self {
            cwd: std::env::current_dir().unwrap_or(PathBuf::from(".")),
            status_fmt: StatusFmt::Normal,
            op: OpType::Bypass,
            git_root: None,
            xargs_cmd: None,
        }
    }

    /// uses self.cwd to find the nearest parent repository
    pub fn repo(&self) -> Option<git2::Repository> {
        git2::Repository::open_ext(
            &self.cwd,
            git2::RepositoryOpenFlags::empty(),
            Vec::<PathBuf>::new(),
        )
        .ok()
    }

    /// get the cache file to read/write file indices to
    pub fn cache_file(&self) -> Option<PathBuf> {
        Some(self.repo()?.path().join(CACHE_FILE))
    }

    /// Locate the root of the git workspace, and update self
    pub fn set_git_root(&mut self) {
        if let Some(repo) = self.repo() {
            self.git_root = repo.workdir().map(|v| v.into());
        }
    }

    pub fn use_cwd<T: AsRef<std::path::Path>>(&self, tail: T) -> PathBuf {
        self.cwd.join(tail)
    }
}
