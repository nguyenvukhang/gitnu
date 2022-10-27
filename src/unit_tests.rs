use crate::{parser::parse, Op, Opts};
use std::{fmt, path::PathBuf, process::Command};

// {{{

#[cfg(test)]
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

#[cfg(test)]
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

#[cfg(test)]
fn iter(strs: Vec<&str>) -> impl Iterator<Item = String> {
    let vec: Vec<String> = strs.iter().map(|v| String::from(*v)).collect();
    vec.into_iter()
}

#[cfg(test)]
fn vec(strs: &[&str]) -> Vec<String> {
    strs.iter().map(|v| String::from(*v)).collect()
}

#[cfg(test)]
fn expect(cmd: (&str, &[&str]), cwd_op: (&str, Op), pargs: &[&str]) -> Opts {
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

#[cfg(test)]
fn _parse(args: &[&str], path: &str) -> Opts {
    parse(iter([&["gitnu"], args].concat()), PathBuf::from(path))
}

// }}}

const CSA: &str = "color.status=always";

#[cfg(test)]
fn setup() {
    std::env::set_current_dir("/tmp").ok();
}

#[test]
fn parse_no_ops() {
    setup();
    // no-ops
    assert_eq!(
        _parse(&["-C", "/tmp"], "/home"),
        expect(("git", &["-C", "/tmp"]), ("/tmp", Op::Unset), &["-C", "/tmp"]),
    );
}

#[test]
fn parse_status() {
    setup();
    // gitnu <options> status
    assert_eq!(
        _parse(&["status"], "/tmp"),
        expect(
            ("git", &["-c", CSA, "status"]),
            ("/tmp", Op::Status(true)),
            &[]
        ),
    );
    assert_eq!(
        _parse(&["-C", "/tmp", "status"], "/home"),
        expect(
            ("git", &["-C", "/tmp", "-c", CSA, "status"]),
            ("/tmp", Op::Status(true)),
            &["-C", "/tmp"]
        ),
    );
}

#[test]
fn parse_enumerate() {
    setup();
    // gitnu <command> <numbers>
    assert_eq!(
        _parse(&["add", "1"], "/home"),
        expect(("git", &["add", "1"]), ("/home", Op::Number), &[]),
    );
    assert_eq!(
        _parse(&["add", "2-4"], "/home"),
        expect(("git", &["add", "2", "3", "4"]), ("/home", Op::Number), &[]),
    );
    assert_eq!(
        _parse(&["add", "8", "2-4"], "/home"),
        expect(
            ("git", &["add", "8", "2", "3", "4"]),
            ("/home", Op::Number),
            &[]
        ),
    );
    assert_eq!(
        _parse(&["add", "8", "--", "2-4"], "/home"),
        expect(("git", &["add", "8", "--", "2-4"]), ("/home", Op::Number), &[]),
    );
}

#[test]
fn parse_general() {
    setup();
    // all together
    assert_eq!(
        _parse(&["-C", "/tmp", "add", "2-5"], "/home"),
        expect(
            ("git", &["-C", "/tmp", "add", "2", "3", "4", "5"]),
            ("/tmp", Op::Number),
            &["-C", "/tmp"]
        ),
    );
}
