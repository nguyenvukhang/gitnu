use crate::opts::{OpType, Opts, StatusFmt};
use std::path::PathBuf;

fn set_op(next: OpType, cur: &mut OpType) {
    match cur {
        OpType::Bypass => *cur = next,
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
    (args, opts)
}
