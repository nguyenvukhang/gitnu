use crate::opts::{Op, Opts, StatusFmt};
use std::path::PathBuf;

fn set_op(next: Op, cur: &mut Op) {
    match cur {
        Op::Bypass => *cur = next,
        _ => (),
    }
}

pub fn parse(args: &Vec<String>) -> (Vec<String>, Opts) {
    let mut opts = Opts::new();
    let mut it = args.iter();
    let mut args: Vec<String> = Vec::new();
    let mut push = |a| args.push(String::from(a));

    it.next();
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "add" | "reset" | "diff" | "checkout" => {
                set_op(Op::Read, &mut opts.op);
                push(arg)
            }
            "status" => {
                set_op(Op::Status, &mut opts.op);
                push(arg)
            }
            "-c" => match it.next() {
                Some(cmd) => {
                    set_op(Op::Xargs, &mut opts.op);
                    opts.xargs_cmd = Some(cmd.into());
                }
                None => push(arg),
            },
            "-C" => match it.next() {
                Some(dir) => opts.cwd = PathBuf::from(dir),
                None => push(arg),
            },
            "--short" | "-s" | "--porcelain" => {
                match opts.op {
                    Op::Status => opts.status_fmt = StatusFmt::Short,
                    _ => (),
                };
                push(arg)
            }
            _ => push(arg),
        }
    }
    opts.set_git_root();
    (args, opts)
}
