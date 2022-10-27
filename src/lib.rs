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
    pub fn cache(&self) -> Option<File> {
        let mut git = Command::new("git");
        git.args(&self.pargs);
        git.args(["rev-parse", "--git-dir"]);
        let git = git.output().ok()?;

        // only take non-empty outputs
        let stdout = match String::from_utf8_lossy(&git.stdout).trim() {
            "" => return None,
            v => PathBuf::from(v),
        };

        // git.stdout returns the git-dir relative to cwd,
        // so prepend it with current dir
        let dir = self.cwd.join(git.status.success().then_some(stdout)?);

        match &self.op {
            Op::Status(_) => File::create(dir.join("gitnu.txt")).ok(),
            _ => File::open(dir.join("gitnu.txt")).ok(),
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

pub fn run(opts: Opts) -> Option<()> {
    fn spawn(mut c: Command) -> Option<()> {
        c.spawn().ok()?.wait().map(|_| ()).ok()
    }
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

#[cfg(test)]
mod unit_tests;
