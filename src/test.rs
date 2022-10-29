use crate::{spawn, Op, Opts};
use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn sh(args: &[&str], dir: &str) -> Option<()> {
    if !PathBuf::from(dir).is_dir() || args.len() == 0 {
        return None;
    }
    let mut cmd = Command::new(args[0]);
    cmd.current_dir(dir).stdout(Stdio::piped());
    cmd.args(&args[1..]);
    spawn(cmd)
}

pub fn mkdir(dir: &str) -> Option<()> {
    match PathBuf::from(dir).is_absolute() {
        true => fs::create_dir_all(dir).ok(),
        false => None,
    }
}

#[allow(dead_code)]
pub fn debug_stdout(cmd: &mut Command) {
    match cmd.output() {
        Ok(v) => eprintln!("{:?}", String::from_utf8_lossy(&v.stdout)),
        Err(_) => (),
    }
}

impl PartialEq for Opts {
    fn eq(&self, other: &Self) -> bool {
        let op = self.op == other.op;
        let cwd = self.cwd.eq(&other.cwd);
        let pargs = self.pargs == other.pargs;
        let cmd = {
            let (a, b) = (&self.cmd, &other.cmd);
            let args = a.get_args().eq(b.get_args());
            let prgm = a.get_program().eq(b.get_program());
            let cwd = {
                let (a, b) = (a.get_current_dir(), b.get_current_dir());
                match (a.is_none(), b.is_none()) {
                    (false, false) => a.unwrap() == b.unwrap(),
                    (true, true) => true,
                    _ => false,
                }
            };
            args && prgm && cwd
        };
        op && cwd && cmd && pargs
    }
}

impl fmt::Debug for Opts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Opts")
            .field("COMMAND", &self.cmd)
            .field("COMMAND_CWD", &self.cmd.get_current_dir())
            .field("CURRENT_DIR", &self.cwd)
            .field("OPERATION", &self.op)
            .field("PRE_ARGS", &self.pargs)
            .finish()
    }
}

pub fn iter(strs: Vec<&str>) -> impl Iterator<Item = String> {
    let vec: Vec<String> = strs.iter().map(|v| String::from(*v)).collect();
    vec.into_iter()
}

pub fn vec(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|v| String::from(*v)).collect()
}

pub fn opts(cmd: (&str, &[&str]), cwd_op: (&str, Op), pargs: &[&str]) -> Opts {
    let mut c = Command::new(cmd.0);
    c.args(cmd.1);
    c.current_dir(cwd_op.0);
    Opts {
        pargs: vec(pargs),
        cwd: PathBuf::from(cwd_op.0),
        op: cwd_op.1,
        cmd: c,
    }
}
