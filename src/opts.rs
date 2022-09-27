// parses opts from arguments
// (determines the state of the app from CLI args)

use std::io::{Error, ErrorKind};
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

impl Opts {
    /// get git command loaded with arg_dir
    fn git_cmd(&self) -> Command {
        let mut git = Command::new("git");
        git.current_dir(&self.arg_dir);
        git
    }

    /// get xargs cmd loaded with cwd
    fn xargs_cmd(&self) -> Result<Command, Error> {
        let cmd = self.xargs_cmd.as_ref().ok_or(Error::new(
            ErrorKind::NotFound,
            "xargs command not found",
        ))?;
        let mut cmd = Command::new(cmd);
        cmd.current_dir(&self.arg_dir);
        Ok(cmd)
    }

    /// get either `git` or the xargs cmd,
    /// depending on the operation type
    pub fn cmd(&self) -> Result<Command, Error> {
        use OpType::*;
        match self.op {
            Read | Status | Bypass => Ok(self.git_cmd()),
            Xargs => self.xargs_cmd(),
        }
    }

    /// uses self.arg_dir to find the nearest parent repository
    fn open_repo(&self) -> Result<git2::Repository, Error> {
        Ok(git2::Repository::open_ext(
            &self.arg_dir,
            git2::RepositoryOpenFlags::empty(),
            Vec::<PathBuf>::new(),
        )
        .map_err(|_| Error::new(ErrorKind::NotFound, "repository not found"))?)
    }

    /// get the cache file to read/write file indices to
    /// filename is gitnu.txt
    pub fn cache_file(&self) -> Result<PathBuf, Error> {
        Ok(self.open_repo()?.path().join("gitnu.txt"))
    }

    /// locate the root of the git workspace
    fn set_git_root(&mut self) {
        if let Ok(repo) = self.open_repo() {
            self.git_root = repo.workdir().map(|v| v.into());
        }
    }
}

pub fn get(args: &Vec<String>) -> (Vec<String>, Opts) {
    let mut opts = Opts {
        op: OpType::Bypass,
        xargs_cmd: None,
        arg_dir: PathBuf::from("."),
        git_root: None,
    };
    let mut set_op_once = |new_op: OpType| {
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
                set_op_once(OpType::Read);
                push()
            }
            "status" => {
                set_op_once(OpType::Status);
                push()
            }
            "-c" => match it.next() {
                Some(cmd) => {
                    set_op_once(OpType::Xargs);
                    opts.xargs_cmd = Some(cmd.to_owned());
                }
                None => push(),
            },
            "-C" => match it.next() {
                Some(dir) => opts.arg_dir = PathBuf::from(dir),
                None => push(),
            },
            _ => push(),
        }
    }
    opts.set_git_root();
    return (res, opts);
}

#[cfg(test)]
fn expected(
    arg_dir: Option<&str>,
    xargs_cmd: Option<&str>,
    op: OpType,
) -> Opts {
    Opts {
        arg_dir: match arg_dir {
            Some(v) => PathBuf::from(v),
            None => PathBuf::from("."),
        },
        xargs_cmd: match xargs_cmd {
            Some(v) => Some(v.to_owned()),
            None => None,
        },
        git_root: None,
        op,
    }
}

#[cfg(test)]
fn received(args: &[&str]) -> Opts {
    let a: Vec<String> = args.iter().map(|v| String::from(*v)).collect();
    let (_, opts) = get(&a);
    opts
}

#[test]
fn test_get_opts() {
    fn assert_eq(rec: Opts, exp: Opts) {
        assert_eq!(rec.arg_dir, exp.arg_dir);
        assert_eq!(rec.xargs_cmd, exp.xargs_cmd);
        assert_eq!(rec.op, exp.op);
    }
    // set arg_dir
    let rec = received(&["-C", "/dev/null"]);
    let exp = expected(Some("/dev/null"), None, OpType::Bypass);
    assert_eq(rec, exp);

    // set xargs_cmd
    let rec = received(&["-c", "nvim"]);
    let exp = expected(None, Some("nvim"), OpType::Xargs);
    assert_eq(rec, exp);

    // set both arg_dir and xargs_cmd
    let rec = received(&["-C", "/etc", "-c", "nvim"]);
    let exp = expected(Some("/etc"), Some("nvim"), OpType::Xargs);
    assert_eq(rec, exp);

    // set both xargs_cmd and arg_dir
    let rec = received(&["-c", "nvim", "-C", "/etc"]);
    let exp = expected(Some("/etc"), Some("nvim"), OpType::Xargs);
    assert_eq(rec, exp);

    // status mode
    let rec = received(&["status", "--short"]);
    let exp = expected(None, None, OpType::Status);
    assert_eq(rec, exp);

    // read mode
    let rec = received(&["add", "2-4"]);
    let exp = expected(None, None, OpType::Read);
    assert_eq(rec, exp);

    // read mode with arg_dir
    let rec = received(&["-C", "/tmp", "add", "2-4"]);
    let exp = expected(Some("/tmp"), None, OpType::Read);
    assert_eq(rec, exp);

    // -C flag without value
    let rec = received(&["-C"]);
    let exp = expected(None, None, OpType::Bypass);
    assert_eq(rec, exp);

    // -C flag with unexpected value
    // (pass on to git)
    let rec = received(&["-C", "status"]);
    let exp = expected(Some("status"), None, OpType::Bypass);
    assert_eq(rec, exp);

    // -c flag without value
    let rec = received(&["-c"]);
    let exp = expected(None, None, OpType::Bypass);
    assert_eq(rec, exp);

    // -c flag with unexpected value
    // (just run in anyways)
    let rec = received(&["-c", "status"]);
    let exp = expected(None, Some("status"), OpType::Xargs);
    assert_eq(rec, exp);
}
