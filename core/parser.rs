use crate::git;
use crate::{App, Cache, Subcommand::*};
use std::path::PathBuf;

/// Parses a string into an inclusive range.
/// "5"   -> Some([5, 5])
/// "2-6" -> Some([2, 6])
/// "foo" -> None
pub fn parse_range(arg: &str) -> Option<[usize; 2]> {
    let (mut i, mut p, mut ok) = (0, [0, 0], [false; 2]);
    let arg: &[u8] = arg.as_bytes();
    let len = arg.len();
    for x in 0..len {
        match arg[x] {
            b'0'..=b'9' => {
                p[i] = p[i] * 10 + (arg[x] - 48) as usize;
                ok[i] |= true;
            }
            b'-' if i == 0 && 0 < x && x < len - 1 => i += 1,
            _ => return None,
        }
    }
    match ok {
        [true, false] => Some([p[0], p[0]]),
        [true, true] => {
            p.sort_unstable();
            Some(p)
        }
        _ => None,
    }
}

/// parse arguments before the git command
/// for a list of all git commands, see ./git_cmd.rs
fn pre_cmd<I: Iterator<Item = String>>(args: &mut I, app: &mut App) {
    let git_cmd = git::subcommands();
    while let Some(arg) = args.next() {
        let arg = arg.as_str();
        if git_cmd.contains(arg) {
            app.set_subcommand(match arg {
                "status" => Status(true),
                _ => Number,
            });
            app.arg(arg);
            break;
        }
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
            app.set_subcommand(Version);
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
    app
}
