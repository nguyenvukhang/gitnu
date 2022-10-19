mod files;
mod status;
use std::{env::current_dir as cwd, path::PathBuf, process::Command};

#[derive(Debug, PartialEq)]
pub enum Op {
    Status(bool),    // gitnu status (true: normal, false: short)
    Number(PathBuf), // gitnu -c nvim 2 / gitnu add 2-4
}

pub struct Opts {
    pub op: Op,
    pub cwd: PathBuf,
}

impl Opts {
    pub fn cache_file(&self) -> Option<PathBuf> {
        let mut t = Command::new("git");
        t.args(["rev-parse", "--git-dir"]).current_dir(&self.cwd);
        let output = t.output().ok()?;
        let t = String::from_utf8_lossy(&output.stdout);
        Some(PathBuf::from(t.trim_end()).join("gitnu.txt"))
    }
}

pub fn parse(args: Vec<String>) -> (Vec<String>, Opts) {
    let (mut res, mut iter, p) = (Vec::new(), args.iter(), |a: &str| a.into());
    let mut o = Opts { cwd: cwd().unwrap_or(p(".")), op: Op::Number(p("git")) };
    iter.next();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "status" => o.op = Op::Status(true),
            "--short" | "-s" | "--porcelain" => match o.op {
                Op::Status(_) => o.op = Op::Status(false),
                _ => (),
            },
            "-c" | "-C" => {
                match iter.next() {
                    Some(v) => match arg.as_str() {
                        "-c" => o.op = Op::Number(PathBuf::from(v)),
                        _ => o.cwd = PathBuf::from(v),
                    },
                    _ => (),
                }
                continue;
            }
            _ => (),
        }
        res.push(arg.to_string());
    }
    (res, o)
}

pub fn core(args: Vec<String>) -> (Vec<PathBuf>, Opts) {
    let (args, opts) = parse(args);
    (files::load(args, &opts), opts)
}

pub fn run(args: Vec<PathBuf>, opts: Opts) -> Option<()> {
    match opts.op {
        Op::Status(_) => status::run(args, opts),
        Op::Number(cmd) => {
            let mut cmd = Command::new(cmd);
            cmd.args(args).spawn().ok()?.wait().map(|_| ()).ok()
        }
    }
}

#[cfg(test)]
mod tests;
