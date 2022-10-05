use std::io::{Error, ErrorKind::NotFound};
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, PartialEq)]
pub enum Op {
    Status, // gitnu status
    Read,   // gitnu add 2-4
    Bypass, // gitnu log --oneline
    Xargs,  // gitnu -c nvim 2
}

pub enum StatusFmt {
    Normal, // gitnu status
    Short,  // gitnu status --short || gitnu status --porcelain
}

pub struct Opts {
    pub xargs_cmd: Option<PathBuf>,
    pub op: Op,
    pub status_fmt: StatusFmt,
    pub git_root: Option<PathBuf>,
    pub cwd: PathBuf,
}

pub fn get_cmd(opts: &Opts) -> Option<Command> {
    let mut cmd = match opts.op {
        Op::Xargs => Command::new(opts.xargs_cmd.as_ref()?),
        _ => Command::new("git"),
    };
    cmd.current_dir(&opts.cwd);
    Some(cmd)
}

pub fn run(cmd: Option<Command>, args: Vec<PathBuf>) -> Result<(), Error> {
    let mut cmd = cmd.ok_or(Error::new(NotFound, "Command not found"))?;
    cmd.args(args).spawn()?.wait().map(|_| ())
}

fn repo(cwd: &PathBuf) -> Option<git2::Repository> {
    use git2::{Repository as R, RepositoryOpenFlags as ROF};
    R::open_ext(cwd, ROF::empty(), Vec::<PathBuf>::new()).ok()
}

impl Opts {
    pub fn new() -> Self {
        Self {
            cwd: std::env::current_dir().unwrap_or(PathBuf::from(".")),
            status_fmt: StatusFmt::Normal,
            op: Op::Bypass,
            git_root: None,
            xargs_cmd: None,
        }
    }

    pub fn cache_file(&self) -> Option<PathBuf> {
        Some(repo(&self.cwd)?.path().join("gitnu.txt"))
    }

    pub fn set_git_root(&mut self) {
        let repo = repo(&self.cwd);
        self.git_root = repo.map(|r| r.workdir().map(|p| p.into())).flatten();
    }
}
