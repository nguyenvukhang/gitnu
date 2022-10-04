#![cfg(test)]
use crate::opts::{OpType, Opts};
use crate::parser;
use std::path::PathBuf;

#[test]
fn test_get_opts() {
    fn ex(c: Option<&str>, x: Option<&str>, op: OpType) -> Opts {
        let mut opts = Opts::new();
        (opts.xargs_cmd, opts.op) = (x.map(String::from), op);
        opts.cwd =
            c.map(PathBuf::from).unwrap_or(std::env::current_dir().unwrap());
        opts
    }
    fn rc(args: &[&str]) -> Opts {
        let args = [&["gitnu"], args].concat();
        parser::parse(&args.iter().map(|v| String::from(*v)).collect()).1
    }
    fn assert_eq((rec, exp): &(&[&str], Opts)) {
        let rec = rc(rec);
        assert_eq!(rec.cwd, exp.cwd);
        assert_eq!(rec.xargs_cmd, exp.xargs_cmd);
        assert_eq!(rec.op, exp.op);
    }

    let tests: &[(&[&str], Opts)] = &[
        // set cwd
        (&["-C", "/dev/null"], ex(Some("/dev/null"), None, OpType::Bypass)),
        // set xargs_cmd
        (&["-c", "nvim"], ex(None, Some("nvim"), OpType::Xargs)),
        // set both cwd and xargs_cmd
        (
            &["-C", "/etc", "-c", "nvim"],
            ex(Some("/etc"), Some("nvim"), OpType::Xargs),
        ),
        // set both xargs_cmd and cwd
        (
            &["-c", "nvim", "-C", "/etc"],
            ex(Some("/etc"), Some("nvim"), OpType::Xargs),
        ),
        // status mode
        (&["status", "--short"], ex(None, None, OpType::Status)),
        // read mode
        (&["add", "2-4"], ex(None, None, OpType::Read)),
        // read mode with cwd
        (&["-C", "/tmp", "add", "2-4"], ex(Some("/tmp"), None, OpType::Read)),
        // -C flag without value
        (&["-C"], ex(None, None, OpType::Bypass)),
        // -C flag with unexpected value (pass on to git)
        (&["-C", "status"], ex(Some("status"), None, OpType::Bypass)),
        // -c flag without value
        (&["-c"], ex(None, None, OpType::Bypass)),
        // -c flag with unexpected value (just run it anyways)
        (&["-c", "status"], ex(None, Some("status"), OpType::Xargs)),
    ];
    for i in tests {
        assert_eq(i);
    }
}
