#![cfg(test)]
use crate::{parse, Op, Opts};
use std::path::PathBuf;

#[test]
fn test_get_opts() {
    fn ex(c: &str, op: Op) -> Opts {
        let mut opts = parse(vec![]).1;
        opts.op = op;
        opts.cwd = match c {
            "" => std::env::current_dir().unwrap(),
            v => PathBuf::from(v),
        };
        opts
    }
    fn rc(args: &[&str]) -> Opts {
        let args = [&["gitnu"], args].concat();
        parse(args.iter().map(|v| String::from(*v)).collect()).1
    }
    fn assert_eq((rec, exp): &(&[&str], Opts)) {
        let rec = rc(rec);
        assert_eq!(rec.cwd, exp.cwd);
        assert_eq!(rec.op, exp.op);
    }

    let p = |s: &str| PathBuf::from(s);
    let tests: &[(&[&str], Opts)] = &[
        // set cwd
        (&["-C", "/dev/null"], ex("/dev/null", Op::Number(p("git")))),
        // set xargs_cmd
        (&["-c", "nvim"], ex("", Op::Number(p("nvim")))),
        // set both cwd and xargs_cmd
        (&["-C", "/etc", "-c", "nvim"], ex("/etc", Op::Number(p("nvim")))),
        // set both xargs_cmd and cwd
        (&["-c", "nvim", "-C", "/etc"], ex("/etc", Op::Number(p("nvim")))),
        // status mode
        (&["status", "--short"], ex("", Op::Status(false))),
        (&["status", "--porcelain"], ex("", Op::Status(false))),
        (&["status"], ex("", Op::Status(true))),
        // read mode
        (&["add", "2-4"], ex("", Op::Number(p("git")))),
        // read mode with cwd
        (&["-C", "/tmp", "add", "2-4"], ex("/tmp", Op::Number(p("git")))),
        // -C flag without value
        (&["-C"], ex("", Op::Number(p("git")))),
        // -C flag with unexpected value (pass on to git)
        (&["-C", "status"], ex("status", Op::Number(p("git")))),
        // -c flag without value
        (&["-c"], ex("", Op::Number(p("git")))),
        // -c flag with unexpected value (just run it anyways)
        (&["-c", "status"], ex("", Op::Number(p("status")))),
    ];
    for i in tests {
        assert_eq(i);
    }
}
