use crate::opts::{OpType, Opts, Parser};
use std::path::PathBuf;

impl Parser for Opts {
    fn parse(args: &Vec<String>) -> (Vec<String>, Opts) {
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
        let mut push = |a| res.push(String::from(a));

        while let Some(arg) = it.next() {
            match arg.as_str() {
                "add" | "reset" | "diff" | "checkout" => {
                    set_op_once(OpType::Read);
                    push(arg)
                }
                "status" => {
                    set_op_once(OpType::Status);
                    push(arg)
                }
                "-c" => match it.next() {
                    Some(cmd) => {
                        set_op_once(OpType::Xargs);
                        opts.xargs_cmd = Some(cmd.to_owned());
                    }
                    None => push(arg),
                },
                "-C" => match it.next() {
                    Some(dir) => opts.arg_dir = PathBuf::from(dir),
                    None => push(arg),
                },
                _ => push(arg),
            }
        }
        opts.set_git_root();
        (res, opts)
    }
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
    let (_, opts) = Opts::parse(&a);
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
