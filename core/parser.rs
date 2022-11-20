use crate::git;
use crate::{App, Subcommand};
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
    app: &mut App,
) -> Vec<String> {
    let git_cmd = git::subcommands();
    let mut cache: Vec<String> = Vec::new();
    while let Some(arg) = args.next() {
        let arg = arg.as_str();
        // set git sub-cmd
        if git_cmd.contains(arg) {
            if arg.eq("status") {
                app.set_once(Subcommand::Status(true));
            } else {
                app.set_once(Subcommand::Number);
                app.read_cache(&mut cache);
            }
            app.cmd.arg(arg);
            break;
        }
        // set cwd using git's -C flag
        if arg.eq("-C") {
            if let Some(cwd) = args.peek() {
                app.cwd = PathBuf::from(cwd);
                app.cmd.current_dir(cwd);
            }
        }
        // cut to displaying version
        if arg.eq("--version") {
            app.subcommand = Subcommand::Version;
            app.cmd = Command::new("git");
            app.cmd.arg("--version");
            return Vec::new();
        }
        app.pargs.push(arg.to_string());
        app.cmd.arg(arg);
    }
    cache
}

/// parse arguments after the git command
/// for a list of all git commands, see ./git_cmd.rs
fn post_cmd(
    args: &mut Peekable<impl Iterator<Item = String>>,
    app: &mut App,
    cache: Vec<String>,
) {
    let mut skip = false;
    while let Some(arg) = args.next() {
        let arg = arg.as_str();
        if arg.eq("--") {
            app.cmd.arg(arg);
            break;
        }
        if ["--short", "-s", "--porcelain"].contains(&arg) {
            app.set_once(Subcommand::Status(false));
        }
        // try to parse argument as a range
        let isf = arg.starts_with('-') && !arg.starts_with("--"); // is short flag
        match (!skip && !isf, parse_range(arg)) {
            (true, Some([s, e])) => (s..e + 1).for_each(|n| {
                app.cmd.arg(cache.get(n).unwrap_or(&n.to_string()));
            }),
            _ => {
                app.cmd.arg(arg);
            }
        }
        skip = isf;
    }
    app.cmd.args(args);
}

pub fn parse(args: impl Iterator<Item = String>, cwd: PathBuf) -> App {
    use Subcommand::*;
    let mut app = App {
        subcommand: Unset,
        cmd: Command::new("git"),
        pargs: Vec::new(),
        cwd,
    };
    if atty::is(atty::Stream::Stdout) {
        app.cmd.args(["-c", "color.ui=always"]);
    }
    app.cmd.current_dir(&app.cwd);
    let mut args = args.skip(1).peekable();
    let cache = pre_cmd(&mut args, &mut app);
    match app.subcommand {
        Status(_) | Number => post_cmd(&mut args, &mut app, cache),
        Unset | Version => (),
    }
    app
}
