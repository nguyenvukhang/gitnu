use crate::git;
use crate::{App, Cache, Subcommand};
use std::path::PathBuf;

fn parse_range(arg: &str) -> Option<[usize; 2]> {
    arg.parse().map(|v| Some([v, v])).unwrap_or_else(|_| {
        let (a, b) = arg.split_once("-")?;
        let a: usize = a.parse().ok()?;
        Some(b.parse().map(|b| [a.min(b), a.max(b)]).unwrap_or([a, a]))
    })
}

/// parse arguments before the git command
/// for a list of all git commands, see ./git_cmd.rs
fn pre_cmd<I: Iterator<Item = String>>(args: &mut I, app: &mut App) {
    let git_cmd = git::subcommands();
    while let Some(arg) = args.next() {
        let arg = arg.as_str();
        // set git sub-cmd
        if git_cmd.contains(arg) {
            app.set_subcommand(match arg {
                "status" => Subcommand::Status(true),
                _ => Subcommand::Number,
            });
            app.arg(arg);
            break;
        }
        // set cwd using git's -C flag
        if arg.eq("-C") {
            app.arg(arg);
            if let Some(cwd) = args.next() {
                app.cwd = PathBuf::from(&cwd);
                app.cmd.current_dir(&cwd);
                app.arg(&cwd);
            }
            continue;
        }
        if arg.eq("--version") {
            app.set_subcommand(Subcommand::Version);
        }
        app.arg(arg);
    }
    app.load_cache_buffer();
}

/// parse arguments after the git command
/// for a list of all git commands, see ./git_cmd.rs
fn post_cmd<I: Iterator<Item = String>>(args: &mut I, app: &mut App) {
    let mut skip = false;
    while let Some(arg) = args.next() {
        let arg = arg.as_str();
        if arg.eq("--") {
            app.arg(arg);
            break;
        }
        if ["--short", "-s", "--porcelain"].contains(&arg) {
            app.set_subcommand(Subcommand::Status(false));
        }
        // try to parse argument as a range
        let isf = arg.starts_with('-') && !arg.starts_with("--"); // is short flag
        match (!skip && !isf, parse_range(arg)) {
            (true, Some([s, e])) => app.load_range(s, e),
            _ => app.arg(arg),
        }
        skip = isf;
    }
    app.cmd.args(args);
}

pub fn parse<I: Iterator<Item = String>>(args: I, cwd: PathBuf) -> App {
    use Subcommand::*;
    let mut app = App::new(cwd);
    let args = &mut args.skip(1);
    pre_cmd(args, &mut app);
    match app.subcommand {
        Status(_) | Number => (),
        Unset | Version => return app,
    }
    post_cmd(args, &mut app);
    app
}
