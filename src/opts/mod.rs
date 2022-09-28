mod actions;
mod commands;
mod parser;

use std::io::Error;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

pub const LIMIT: usize = 50;
pub type Cache = Vec<Option<PathBuf>>;

#[derive(Debug, PartialEq, Clone)]
pub enum OpType {
    Status, // gitnu status
    Read,   // gitnu add 2-4
    Bypass, // gitnu log --oneline
    Xargs,  // gitnu -c nvim 2
}

#[derive(Debug, PartialEq, Clone)]
pub enum StatusFmt {
    Normal, // gitnu status
    Short,  // gitnu status --short || gitnu status --porcelain
}

#[derive(Debug, PartialEq, Clone)]
pub struct Opts {
    pub xargs_cmd: Option<String>,
    pub op: OpType,
    pub status_fmt: StatusFmt,

    /// result of git rev-parse --show-toplevel
    /// root of git workspace
    pub git_root: Option<PathBuf>,

    /// taken as a CLI argument with:
    /// gitnu -C <arg_dir>
    ///
    /// defaults to current working directory
    pub arg_dir: PathBuf,
}

pub trait Parser {
    /// The gitnu options parser
    /// parses Opts from command line arguments
    fn parse(args: &Vec<String>) -> (Vec<String>, Opts);
}

pub trait Commands {
    /// get either `git` or the xargs cmd,
    /// depending on the operation type
    fn cmd(&self) -> Option<Command>;
}

pub trait CacheActions {
    fn write_cache(&self, content: String) -> Option<()>;
    fn read_cache(&self) -> Option<Cache>;
}

pub trait RunAction {
    fn run(&self, args: Vec<PathBuf>) -> Result<ExitStatus, Error>;
}

impl Opts {
    /// uses self.arg_dir to find the nearest parent repository
    pub fn open_repo(&self) -> Option<git2::Repository> {
        git2::Repository::open_ext(
            &self.arg_dir,
            git2::RepositoryOpenFlags::empty(),
            Vec::<PathBuf>::new(),
        )
        .ok()
    }

    /// get the cache file to read/write file indices to
    /// filename is gitnu.txt
    pub fn cache_file(&self) -> Option<PathBuf> {
        Some(self.open_repo()?.path().join("gitnu.txt"))
    }

    /// locate the root of the git workspace
    pub fn set_git_root(&mut self) {
        if let Some(repo) = self.open_repo() {
            self.git_root = repo.workdir().map(|v| v.into());
        }
    }
}
