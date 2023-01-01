use crate::git;
use crate::{App, Cache, Subcommand::*};
use std::path::PathBuf;

/// Parses a string into an inclusive range.
/// "5"   -> Some([5, 5])
/// "2-6" -> Some([2, 6])
/// "foo" -> None
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
    let cmdr = git::Commander::new();
    while let Some(arg) = args.next() {
        let arg = arg.as_str();

        // obtain subcommand
        if let Some(subcommand) = cmdr.get_subcommand(arg) {
            app.set_subcommand(match subcommand {
                "status" => Status(true),
                _ => Number,
            });
            app.arg(arg);
            break;
        } else if arg.eq("--version") {
            app.set_subcommand(Version);
            app.arg(arg);
            break;
        }

        // set app cwd using the -C flag
        if arg.eq("-C") {
            app.arg(arg);
            if let Some(cwd) = args.next() {
                app.cmd.current_dir(&cwd);
                app.arg(&cwd);
            }
            continue;
        }

        app.arg(arg);
    }
    app.set_argc();
}

/// parse arguments after the git command
/// for a list of all git commands, see ./git_cmd.rs
fn post_cmd<I: Iterator<Item = String>>(args: &mut I, app: &mut App) {
    let mut skip = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--" => {
                app.arg(arg);
                break;
            }
            "--short" | "-s" | "--porcelain" => {
                app.set_subcommand(Status(false))
            }
            _ => (),
        }
        let isf = arg.starts_with('-') && !arg.starts_with("--"); // is short flag
        match (!skip && !isf, parse_range(&arg)) {
            (true, Some([s, e])) => app.load_range(s, e),
            _ => app.arg(arg),
        }
        skip = isf;
    }
    app.cmd.args(args);
}

/// Parses all command-line arguments and returns an App instance that is ready
/// to be ran.
pub fn parse<I: Iterator<Item = String>>(args: I, cwd: PathBuf) -> App {
    let mut app = App::new(cwd);
    let args = &mut args.skip(1);
    pre_cmd(args, &mut app);
    match app.subcommand {
        Status(_) => (),
        Number => app.load_cache_buffer(),
        Unset | Version => return app,
    }
    post_cmd(args, &mut app);
    if let Ok(debug) = std::env::var("DEBUG") {
        if debug.eq("1") {
            println!("{app:?}")
        }
    }
    app
}
