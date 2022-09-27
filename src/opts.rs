use std::path::PathBuf;
use std::process::Command;

pub const LIMIT: usize = 50;

#[derive(Debug, PartialEq, Clone)]
pub enum OpType {
    Status, // gitnu status
    Read,   // gitnu add 2-4
    Bypass, // gitnu log --oneline
    Xargs,  // gitnu -c nvim 2
}

#[derive(Debug, PartialEq, Clone)]
pub struct Opts {
    pub xargs_cmd: Option<String>,
    pub op: OpType,

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

impl Opts {
    /// get git command loaded with arg_dir
    fn git_cmd(&self) -> Command {
        let mut git = Command::new("git");
        git.current_dir(&self.arg_dir);
        git
    }

    /// get xargs cmd loaded with cwd
    fn xargs_cmd(&self) -> Option<Command> {
        let cmd = self.xargs_cmd.as_ref()?;
        let mut cmd = Command::new(cmd);
        cmd.current_dir(&self.arg_dir);
        Some(cmd)
    }

    /// get either `git` or the xargs cmd,
    /// depending on the operation type
    pub fn cmd(&self) -> Option<Command> {
        use OpType::*;
        match self.op {
            Read | Status | Bypass => Some(self.git_cmd()),
            Xargs => self.xargs_cmd(),
        }
    }

    /// uses self.arg_dir to find the nearest parent repository
    fn open_repo(&self) -> Option<git2::Repository> {
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
