// parses opts from arguments
// (determines the state of the app from CLI args)

use crate::shell::get_stdout;
use std::io::{Error, ErrorKind};
use std::process::Command;

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
    pub git_dir: Option<String>,
    pub op: OpType,
}

impl Opts {
    /// get git command loaded with git_dir
    fn git_cmd(&self) -> Result<Command, Error> {
        let mut git = Command::new("git");
        if let Some(git_dir) = &self.git_dir {
            git.arg("-C");
            git.arg(git_dir);
        }
        Ok(git)
    }
    pub fn cmd(&self) -> Result<Command, Error> {
        match self.op {
            OpType::Read | OpType::Status | OpType::Bypass => self.git_cmd(),
            OpType::Xargs => match &self.xargs_cmd {
                Some(xargs_cmd) => Ok(Command::new(xargs_cmd)),
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "xargs command not is None",
                    ))
                }
            },
        }
    }
    fn cache_dir(&self) -> Result<std::path::PathBuf, Error> {
        use std::path::PathBuf;
        let mut git = self.git_cmd()?;
        git.args(["rev-parse", "--path-format=absolute", "--git-dir"]);
        let res = match get_stdout(&mut git) {
            Ok(v) => PathBuf::from(v),
            _ => PathBuf::from("/dev/null"),
        };
        Ok(res)
    }
    fn cache_file(&self) -> Result<std::path::PathBuf, Error> {
        let cache_dir = self.cache_dir()?;
        Ok(cache_dir.join("gitnu.txt"))
    }
    pub fn write_cache(&self, content: String) -> Result<(), Error> {
        let cache_file = self.cache_file()?;
        return std::fs::write(cache_file, content);
    }
    pub fn read_cache(&self) -> Result<Vec<String>, Error> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let cache_file = self.cache_file()?;
        let file = File::open(cache_file)?;
        let res: Vec<String> = BufReader::new(file)
            .lines()
            .filter_map(|v| v.ok())
            .map(|v| v.to_string())
            .collect();
        Ok(res)
    }
}

pub fn get(args: &Vec<String>) -> (Opts, Vec<String>) {
    let mut opts = Opts { xargs_cmd: None, git_dir: None, op: OpType::Bypass };
    let mut set_op = |new_op: OpType| {
        if opts.op == OpType::Bypass {
            opts.op = new_op;
        }
    };
    let mut res: Vec<String> = Vec::new();
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        let mut push = || res.push(String::from(arg));
        match arg.as_str() {
            "add" | "reset" | "diff" | "checkout" => {
                set_op(OpType::Read);
                push()
            }
            "status" => {
                set_op(OpType::Status);
                push()
            }
            "-c" => {
                set_op(OpType::Xargs);
                if let Some(cmd) = it.next() {
                    opts.xargs_cmd = Some(cmd.to_owned());
                }
            }
            "-C" => {
                if let Some(dir) = it.next() {
                    opts.git_dir = Some(dir.to_owned());
                }
            }
            _ => push(),
        }
    }
    return (opts, res);
}

#[cfg(test)]
fn get_expected(
    git_dir: Option<&str>,
    xargs_cmd: Option<&str>,
    op: OpType,
) -> Opts {
    let stringify = |v: Option<&str>| match v {
        None => None,
        Some(v) => Some(String::from(v)),
    };
    Opts { git_dir: stringify(git_dir), xargs_cmd: stringify(xargs_cmd), op }
}

#[cfg(test)]
fn get_received(args: &[&str]) -> Opts {
    let a: Vec<String> = args.iter().map(|v| v.to_string()).collect();
    let (opts, _) = get(&a);
    opts
}

#[test]
fn test_get_opts() {
    // set git_dir
    let rec = get_received(&["-C", "/dev/null"]);
    let exp = get_expected(Some("/dev/null"), None, OpType::Bypass);
    assert_eq!(rec, exp);

    // set xargs_cmd
    let rec = get_received(&["-c", "nvim"]);
    let exp = get_expected(None, Some("nvim"), OpType::Xargs);
    assert_eq!(rec, exp);

    // set both git_dir and xargs_cmd
    let rec = get_received(&["-C", "/etc", "-c", "nvim"]);
    let exp = get_expected(Some("/etc"), Some("nvim"), OpType::Xargs);
    assert_eq!(rec, exp);

    // set both xargs_cmd and git_dir
    let rec = get_received(&["-c", "nvim", "-C", "/etc"]);
    let exp = get_expected(Some("/etc"), Some("nvim"), OpType::Xargs);
    assert_eq!(rec, exp);

    // status mode
    let rec = get_received(&["status", "--short"]);
    let exp = get_expected(None, None, OpType::Status);
    assert_eq!(rec, exp);

    // read mode
    let rec = get_received(&["add", "2-4"]);
    let exp = get_expected(None, None, OpType::Read);
    assert_eq!(rec, exp);

    // read mode with git_dir
    let rec = get_received(&["-C", "/tmp", "add", "2-4"]);
    let exp = get_expected(Some("/tmp"), None, OpType::Read);
    assert_eq!(rec, exp);
}
