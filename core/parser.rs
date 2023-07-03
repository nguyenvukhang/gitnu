use crate::prelude::*;
use crate::{App, GitCommand};
use std::path::PathBuf;

/// Parses a string into an inclusive range.
/// "5"   -> Some([5, 5])
/// "2-6" -> Some([2, 6])
/// "foo" -> None
fn parse_range(arg: &str) -> Option<[usize; 2]> {
    arg.parse().map(|v| Some([v, v])).unwrap_or_else(|_| {
        let (a, b) = arg.split_once("-")?;
        let a: usize = a.parse().ok()?;
        b.parse().map(|b| [a.min(b), a.max(b)]).ok()
    })
}

/// Parses all command-line arguments and returns an App instance that is ready
/// to be ran.
pub fn parse<I: Iterator<Item = String>>(cwd: PathBuf, args: I) -> Result<App> {
    let mut app = App::new(cwd);
    let args = &mut args.skip(1);

    while let Some(arg) = args.next() {
        if arg.eq("--version") {
            app.git.set_subcommand(GitCommand::Version);
            break;
        }
        if let Some(true) = app.git.arg(&arg) {
            // Fail silently when load_cache returns an Error
            //
            // Should probably deal with this but if loading cache fails,
            // git-nu defaults back to regular git behaviour so there's no
            // immediate impact.
            //
            // TODO: add a user-facing way for git-nu to show git-nu
            // related errors
            drop(app.load_cache());
            break;
        }
    }

    let git_command = match app.git.subcommand().map(|v| v.clone()) {
        None => return Ok(app),
        Some(v) => v,
    };

    let mut skip = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--short" | "-s" | "--porcelain" => {
                app.git.set_subcommand(GitCommand::Status(false))
            }
            _ => (),
        }
        match (skip, parse_range(&arg)) {
            (false, Some([s, e])) => app.load_range(s, e),
            _ => app.git.arg_unchecked(&arg),
        }
        skip = git_command.skip_next_arg(&arg);
    }
    app.git.args_unchecked(args);

    Ok(app)
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
                $test(parse(PathBuf::new(), args).unwrap());
            }
        };
    }

    macro_rules! assert_args {
        ($received:expr, $expected:expr) => {{
            let expected = {
                let mut cmd = Command::new("");
                cmd.args($expected);
                cmd.get_args()
                    .filter_map(|v| v.to_str())
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
            };

            let received = $received.git.get_string_args();
            if !received.eq(&expected) {
                panic!("\nreceived: {:?}\nexpected: {:?}\n", received, expected)
            }
        }};
    }

    test!(test_status, "/home", ["-C", "/tmp", "status"], |app: App| {
        assert_eq!(app.git.subcommand(), Some(&GitCommand::Status(true)));
    });

    test!(test_single, ["add", "1"], |app: App| {
        assert_args!(app, ["add", "1"]);
    });

    test!(test_range, ["add", "2-4"], |app: App| {
        assert_args!(app, ["add", "2", "3", "4"]);
    });

    test!(test_mix, ["add", "8", "2-4"], |app: App| {
        assert_args!(app, ["add", "8", "2", "3", "4"]);
    });

    // Gitnu will not seek to interfere with these cases smartly.
    test!(test_overlap, ["add", "3-5", "2-4"], |app: App| {
        assert_args!(app, ["add", "3", "4", "5", "2", "3", "4"]);
    });

    // anything after `--` will also be processed. This is for commands
    // like `git reset` which requires pathspecs to appear after --.
    test!(test_double_dash, ["add", "3-5", "--", "2-4"], |app: App| {
        assert_args!(app, ["add", "3", "4", "5", "--", "2", "3", "4"]);
    });

    test!(test_zeroes_1, ["add", "0"], |app: App| {
        assert_args!(app, ["add", "0"]);
    });
    test!(test_zeroes_2, ["add", "0-1"], |app: App| {
        assert_args!(app, ["add", "0", "1"]);
    });
    test!(test_zeroes_3, ["add", "0-0"], |app: App| {
        assert_args!(app, ["add", "0"]);
    });

    // Filenames containing dashed dates
    test!(test_date_filename, ["add", "2021-01-31"], |app: App| {
        assert_args!(app, ["add", "2021-01-31"]);
    });
}
