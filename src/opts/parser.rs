use crate::opts::{OpType, Opts, Parser, StatusFmt};
use std::path::PathBuf;

fn set_op(next: OpType, cur: &mut OpType) {
    match cur {
        OpType::Bypass => *cur = next,
        _ => (),
    }
}

impl Parser for Opts {
    fn parse(args: &Vec<String>) -> (Vec<String>, Opts) {
        let mut opts = Opts::new();
        let mut res: Vec<String> = Vec::new();
        let mut it = args.iter();
        let mut push = |a| res.push(String::from(a));

        while let Some(arg) = it.next() {
            match arg.as_str() {
                "add" | "reset" | "diff" | "checkout" => {
                    set_op(OpType::Read, &mut opts.op);
                    push(arg)
                }
                "status" => {
                    set_op(OpType::Status, &mut opts.op);
                    push(arg)
                }
                "-c" => match it.next() {
                    Some(cmd) => {
                        set_op(OpType::Xargs, &mut opts.op);
                        opts.xargs_cmd = Some(cmd.to_owned());
                    }
                    None => push(arg),
                },
                "-C" => match it.next() {
                    Some(dir) => opts.cwd = PathBuf::from(dir),
                    None => push(arg),
                },
                "--short" | "-s" | "--porcelain" => {
                    match opts.op {
                        OpType::Status => opts.status_fmt = StatusFmt::Short,
                        _ => (),
                    };
                    push(arg)
                }
                _ => push(arg),
            }
        }
        opts.set_git_root();
        (res, opts)
    }
}

#[test]
fn test_get_opts() {
    fn ex(c: Option<&str>, x: Option<&str>, op: OpType) -> Opts {
        let mut opts = Opts::new();
        (opts.xargs_cmd, opts.op) = (x.map(String::from), op);
        opts.cwd = PathBuf::from(c.unwrap_or("."));
        opts
    }
    fn rc(args: &[&str]) -> Opts {
        Opts::parse(&args.iter().map(|v| String::from(*v)).collect()).1
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
