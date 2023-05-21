use crate::{App, Cache, GitCommand};
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
    while let Some(arg) = args.next() {
        let arg = arg.as_str();
        if let Ok(git_cmd) = GitCommand::try_from(arg) {
            app.set_git_command(git_cmd);
        } else if arg.eq("--version") {
            app.set_git_command(GitCommand::Version);
        }
        app.arg(arg);
        if app.git_command().is_some() {
            break;
        }
    }
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
                app.set_git_command(GitCommand::Status(false))
            }
            _ => (),
        }
        match (skip, parse_range(&arg)) {
            (false, Some([s, e])) => app.load_range(s, e),
            _ => app.arg(&arg),
        }
        skip = app.git_command().map_or(false, |v| v.skip_next_arg(&arg));
    }
    app.cmd.args(args);
}

/// Parses all command-line arguments and returns an App instance that is ready
/// to be ran.
pub fn parse<I: Iterator<Item = String>>(cwd: PathBuf, args: I) -> App {
    let mut app = App::new(cwd);
    let args = &mut args.skip(1);

    pre_cmd(args, &mut app);

    match app.git_command {
        Some(GitCommand::Status(_)) => {}
        Some(_) => app.load_cache(),
        _ => {}
    }

    post_cmd(args, &mut app);

    app
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;
    use std::process::Command;

    macro_rules! test {
        ($name:ident, $args:expr, $test:expr) => {
            test!($name, env::temp_dir(), $args, $test);
        };
        ($name:ident, $cwd:expr, $args:expr, $test:expr) => {
            #[test]
            fn $name() {
                let mut args = vec![""];
                args.extend($args);
                let args = args.iter().map(|v| v.to_string());
                $test(parse(PathBuf::new(), args));
            }
        };
    }

    macro_rules! assert_args {
        ($received:expr, $expected:expr) => {{
            let expected = {
                let mut cmd = Command::new("");
                if atty::is(atty::Stream::Stdout) {
                    cmd.args(["-c", "color.ui=always"]);
                }
                cmd.args($expected);
                cmd
            };

            if !$received.get_args().eq(expected.get_args()) {
                panic!(
                    "\nreceived: {:?}\nexpected: {:?}\n",
                    $received.get_args(),
                    expected.get_args()
                )
            }
        }};
    }

    test!(test_status, "/home", ["-C", "/tmp", "status"], |app: App| {
        assert_eq!(app.git_command(), Some(&GitCommand::Status(true)));
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
}
