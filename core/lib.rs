use std::io::{BufRead, BufReader, Read};
use std::{fs::File, path::PathBuf, process::Command};
mod git_cmd;
mod parser;
mod status;
pub use parser::parse;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Debug, PartialEq)]
pub enum Op {
    Status(bool), // true: normal, false: short
    Number,
    Unset,
    Version,
}

#[derive(Debug)]
pub struct Opts {
    op: Op,
    cwd: PathBuf,
    cmd: Command,
    pargs: Vec<String>,
}

fn path_from_stdout(out: std::process::Output) -> Option<PathBuf> {
    // don't take value if return code is a failure
    if !out.status.success() {
        return None;
    }
    // only take non-empty outputs
    match String::from_utf8_lossy(&out.stdout).trim() {
        v if v.is_empty() => None,
        v => Some(PathBuf::from(v)),
    }
}

impl Opts {
    /// use the path obtained from `git rev-parse --git-dir` to store the cache.
    /// this is usually the .git folder of regular repositories, and somewhere
    /// deeper for worktrees.
    fn cache_path(&self) -> Option<PathBuf> {
        let git = Command::new("git")
            .args(&self.pargs)
            .args(["rev-parse", "--git-dir"])
            .output()
            .ok()?;
        let relative_path = path_from_stdout(git)?;
        // git.stdout returns the git-dir relative to cwd,
        // so prepend it with current dir
        Some(self.cwd.join(relative_path).join("gitnu.txt"))
    }

    pub fn cache(&self, create: bool) -> Option<File> {
        self.cache_path().and_then(|v| match create {
            true => File::create(v).ok(),
            false => File::open(v).ok(),
        })
    }

    pub fn read_cache(&self) -> Option<Vec<String>> {
        let lines = lines(self.cache(false)?);
        Some(["0"].iter().map(|v| v.to_string()).chain(lines).collect())
    }

    pub fn set_once(&mut self, op: Op) {
        match (&self.op, &op) {
            (Op::Unset, _) => self.op = op,
            (Op::Status(true), Op::Status(false)) => self.op = op,
            _ => (),
        }
    }
}

fn lines<R: Read>(src: R) -> impl Iterator<Item = String> {
    BufReader::new(src).lines().filter_map(|v| v.ok())
}

pub fn spawn(mut c: Command) -> Option<()> {
    c.spawn().ok()?.wait().map(|_| ()).ok()
}

pub fn run(opts: Opts) -> Option<()> {
    match opts.op {
        Op::Status(normal) => status::status(opts, normal),
        Op::Version => {
            let res = spawn(opts.cmd);
            println!("gitnu version {}", VERSION.unwrap_or("unknown"));
            res
        }
        _ => spawn(opts.cmd),
    }
}

mod cache_tests;
mod parser_tests;

#[cfg(test)]
mod test;
