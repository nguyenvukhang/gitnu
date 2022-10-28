use crate::{git_cmd, Op, Opts};
use std::collections::HashSet;
use std::iter::Peekable;
use std::{path::PathBuf, process::Command};

fn parse_range(arg: &str) -> Option<[usize; 2]> {
    arg.parse().map(|v| Some([v, v])).unwrap_or_else(|_| {
        let (a, b) = arg.split_once("-")?;
        let a: usize = a.parse().ok()?;
        Some(b.parse().map(|b| [a.min(b), a.max(b)]).unwrap_or([a, a]))
    })
}

/// parse arguments before the git command
/// for a list of all git commands, see ./git_cmd.rs
fn pre_cmd(
    args: &mut Peekable<impl Iterator<Item = String>>,
    opts: &mut Opts,
) -> Vec<String> {
    let git_cmd = HashSet::from(git_cmd::GIT_CMD);
    let mut cache: Vec<String> = Vec::new();
    while let Some(arg) = args.next() {
        let arg = arg.as_str();
        // set git sub-cmd
        if git_cmd.contains(arg) {
            if arg.eq("status") {
                opts.cmd.args(["-c", "color.status=always"]);
                opts.set_once(Op::Status(true));
            } else {
                opts.set_once(Op::Number);
                cache = opts.read_cache();
            }
            opts.cmd.arg(arg);
            break;
        }
        // set cwd using git's -C flag
        if arg.eq("-C") {
            if let Some(cwd) = args.peek() {
                opts.cwd = PathBuf::from(cwd);
                opts.cmd.current_dir(cwd);
            }
        }
        // cut to displaying version
        if arg.eq("--version") {
            opts.op = Op::Version;
            opts.cmd = Command::new("git");
            opts.cmd.arg("--version");
            return Vec::new();
        }
        opts.pargs.push(arg.to_string());
        opts.cmd.arg(arg);
    }
    cache
}

/// parse arguments after the git command
/// for a list of all git commands, see ./git_cmd.rs
fn post_cmd(
    args: &mut Peekable<impl Iterator<Item = String>>,
    opts: &mut Opts,
    cache: Vec<String>,
) {
    let mut skip = false;
    while let Some(arg) = args.next() {
        let arg = arg.as_str();
        if arg.eq("--") {
            opts.cmd.arg(arg);
            break;
        }
        if ["--short", "-s", "--porcelain"].contains(&arg) {
            opts.set_once(Op::Status(false));
        }
        // try to parse argument as a range
        let isf = arg.starts_with('-') && !arg.starts_with("--"); // is short flag
        match (!skip && !isf, parse_range(arg)) {
            (true, Some([s, e])) => (s..e + 1).for_each(|n| {
                opts.cmd.arg(cache.get(n).unwrap_or(&n.to_string()));
            }),
            _ => {
                opts.cmd.arg(arg);
            }
        }
        skip = isf;
    }
    opts.cmd.args(args);
}

pub fn parse(args: impl Iterator<Item = String>, cwd: PathBuf) -> Opts {
    let mut opts = Opts {
        op: Op::Unset,
        cmd: Command::new("git"),
        pargs: Vec::new(),
        cwd,
    };
    opts.cmd.current_dir(&opts.cwd);
    let mut args = args.skip(1).peekable();
    let cache = pre_cmd(&mut args, &mut opts);
    match opts.op {
        Op::Status(_) | Op::Number => post_cmd(&mut args, &mut opts, cache),
        Op::Unset | Op::Version => (),
    }
    opts
}
