#![cfg(test)]
use crate::opts::{Op, Opts};
use crate::parser;
use std::path::PathBuf;

#[test]
fn test_get_opts() {
    fn ex(c: Option<&str>, x: Option<&str>, op: Op) -> Opts {
        let mut opts = Opts::new();
        (opts.xargs_cmd, opts.op) = (x.map(PathBuf::from), op);
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
        (&["-C", "/dev/null"], ex(Some("/dev/null"), None, Op::Bypass)),
        // set xargs_cmd
        (&["-c", "nvim"], ex(None, Some("nvim"), Op::Xargs)),
        // set both cwd and xargs_cmd
        (
            &["-C", "/etc", "-c", "nvim"],
            ex(Some("/etc"), Some("nvim"), Op::Xargs),
        ),
        // set both xargs_cmd and cwd
        (
            &["-c", "nvim", "-C", "/etc"],
            ex(Some("/etc"), Some("nvim"), Op::Xargs),
        ),
        // status mode
        (&["status", "--short"], ex(None, None, Op::Status)),
        // read mode
        (&["add", "2-4"], ex(None, None, Op::Read)),
        // read mode with cwd
        (&["-C", "/tmp", "add", "2-4"], ex(Some("/tmp"), None, Op::Read)),
        // -C flag without value
        (&["-C"], ex(None, None, Op::Bypass)),
        // -C flag with unexpected value (pass on to git)
        (&["-C", "status"], ex(Some("status"), None, Op::Bypass)),
        // -c flag without value
        (&["-c"], ex(None, None, Op::Bypass)),
        // -c flag with unexpected value (just run it anyways)
        (&["-c", "status"], ex(None, Some("status"), Op::Xargs)),
    ];
    for i in tests {
        assert_eq(i);
    }
}
