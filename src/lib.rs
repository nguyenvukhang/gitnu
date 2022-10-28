use std::io::{BufRead, BufReader, Read};
use std::{fs::File, path::PathBuf, process::Command};
mod git_cmd;
mod parser;
mod status;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Debug, PartialEq)]
pub enum Op {
    Status(bool), // true: normal, false: short
    Number,
    Unset,
    Version,
}

pub struct Opts {
    op: Op,
    cwd: PathBuf,
    cmd: Command,
    pargs: Vec<String>,
}

impl Opts {
    fn cache_path(&self) -> Option<PathBuf> {
        let git = Command::new("git")
            .args(&self.pargs)
            .args(["rev-parse", "--git-dir"])
            .current_dir(&self.cwd)
            .output()
            .ok()?;

        // only take non-empty outputs
        let relative_path = match String::from_utf8_lossy(&git.stdout).trim() {
            v if v.is_empty() => return None,
            v => PathBuf::from(v),
        };

        // git.stdout returns the git-dir relative to cwd,
        // so prepend it with current dir
        let dir = self.cwd.join(git.status.success().then_some(relative_path)?);
        Some(dir.join("gitnu.txt"))
    }

    pub fn cache(&self) -> Option<File> {
        let fp = self.cache_path()?;
        match &self.op {
            Op::Status(_) => File::create(fp).ok(),
            _ => File::open(fp).ok(),
        }
    }

    pub fn read_cache(&self) -> Vec<String> {
        let mut c = vec![String::from("0")];
        self.cache().map(|f| c.extend(lines(f)));
        c
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

pub fn core(args: impl Iterator<Item = String>, cwd: PathBuf) -> Opts {
    parser::parse(args, cwd)
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
