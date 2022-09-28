mod cache;
mod commands;
mod parser;

use std::io::Error;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};

pub const LIMIT: usize = 50;
pub const CACHE_FILE: &str = "gitnu.txt";

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
    /// gitnu -C <cwd>
    ///
    /// defaults to current working directory
    pub cwd: PathBuf,
}

/// holds one method: parse()
/// converts CLI arguments from a String vector to Opts
pub trait Parser {
    /// The gitnu options parser
    /// parses Opts from command line arguments
    fn parse(args: &Vec<String>) -> (Vec<String>, Opts);
}

/// holds two methods: cmd() and run()
/// cmd() gets a command to run
/// run() runs the command with some args
pub trait Commands {
    /// Returns either `git` or the xargs cmd, loaded with current
    /// working directory, depending on the operation type
    fn cmd(&self) -> Option<Command>;

    /// Run and don't look back.
    /// This function executes no logic. Only runs a command with the
    /// supplied arguments.
    fn run(&self, args: Vec<PathBuf>) -> Result<ExitStatus, Error>;
}

/// Read/write operations for the cache file
pub trait CacheOps {
    /// writes to cache
    fn write_cache(&self, content: String) -> Option<()>;

    /// reads cache file to the Cache struct
    fn read_cache(&self) -> Option<Cache>;
}

impl Opts {
    /// Initialize with empty options.
    pub fn new() -> Self {
        Self {
            cwd: PathBuf::from("."),
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
}
