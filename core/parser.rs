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
    let cmdr = git::Commander::new(app.cwd());
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
pub fn parse<I: Iterator<Item = String>>(cwd: PathBuf, args: I) -> App {
    let mut app = App::new(cwd);
    let args = &mut args.skip(1);
    pre_cmd(args, &mut app);
    if app.subcommand == Number {
        app.load_cache_buffer();
    }
    post_cmd(args, &mut app);
    app
}

#[cfg(test)]
use std::path::Path;
#[cfg(test)]
use std::process::Command;

macro_rules! test {
    ($name:ident, $args:expr, $test:expr) => {
        test!($name, std::env::temp_dir(), $args, $test);
    };
    ($name:ident, $cwd:expr, $args:expr, $test:expr) => {
        #[test]
        fn $name() {
            let args = std::iter::once(&"git")
                .chain($args.iter())
                .map(|v| v.to_string());
            let cwd = PathBuf::from($cwd);
            $test(parse(cwd, args));
        }
    };
}

#[cfg(test)]
macro_rules! assert_args {
    ($received:expr, $args:expr) => {{
        let mut expected = Command::new("");
        expected.current_dir(std::env::temp_dir());
        if atty::is(atty::Stream::Stdout) {
            expected.args(["-c", "color.ui=always"]);
        }
        expected.args($args);
        if !$received.get_args().eq(expected.get_args()) {
            panic!(
                "\nreceived: {:?}\nexpected: {:?}\n",
                $received.get_args(),
                expected.get_args()
            )
        }
    }};
}

test!(test_no_ops, "/home", ["-C", "/tmp"], |app: App| {
    assert_eq!(app.cwd(), Path::new("/tmp"));
    assert_eq!(app.subcommand(), &Unset);
});

test!(test_status, "/tmp", ["status"], |app: App| {
    assert_eq!(app.subcommand(), &Status(true));
});

test!(test_status_diff_dir, "/home", ["-C", "/tmp", "status"], |app: App| {
    assert_eq!(app.subcommand(), &Status(true));
    assert_eq!(app.cwd(), Path::new("/tmp"));
});

test!(test_single, ["add", "1"], |app: App| {
    assert_args!(app.cmd(), ["add", "1"]);
});

test!(test_range, ["add", "2-4"], |app: App| {
    assert_args!(app.cmd(), ["add", "2", "3", "4"]);
});

test!(test_mix, ["add", "8", "2-4"], |app: App| {
    assert_args!(app.cmd(), ["add", "8", "2", "3", "4"]);
});

// Gitnu will not seek to interfere with these cases smartly.
test!(test_overlap, ["add", "3-5", "2-4"], |app: App| {
    assert_args!(app.cmd(), ["add", "3", "4", "5", "2", "3", "4"]);
});

// anything after `--` will not be processed.
test!(test_double_dash, ["add", "3-5", "--", "2-4"], |app: App| {
    assert_args!(app.cmd(), ["add", "3", "4", "5", "--", "2-4"]);
});

test!(test_zeroes_1, ["add", "0"], |app: App| {
    assert_args!(app.cmd(), ["add", "0"]);
});
test!(test_zeroes_2, ["add", "0-1"], |app: App| {
    assert_args!(app.cmd(), ["add", "0", "1"]);
});
test!(test_zeroes_3, ["add", "0-0"], |app: App| {
    assert_args!(app.cmd(), ["add", "0"]);
});
