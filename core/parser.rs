use crate::prelude::*;
use crate::{App, GitCommand};

/// Parses a string into an inclusive range.
/// "5"   -> Some([5, 5])
/// "2-6" -> Some([2, 6])
/// "foo" -> None
pub fn range(arg: &str) -> Option<[usize; 2]> {
    arg.parse().map(|v| Some([v, v])).unwrap_or_else(|_| {
        let (a, b) = arg.split_once("-")?;
        let a: usize = a.parse().ok()?;
        b.parse().map(|b| [a.min(b), a.max(b)]).ok()
    })
}

impl App {
    pub fn parse<I>(mut self, args: I) -> Result<Self>
    where
        I: IntoIterator<Item = String>,
    {
        let args = &mut args.into_iter().skip(1);

        while let Some(arg) = args.next() {
            if arg.eq("--version") {
                self.git.set_subcommand(GitCommand::Version);
                break;
            }
            if let Some(true) = self.git.arg(&arg) {
                break;
            }
        }

        let git_command = match self.git.subcommand().map(|v| v.clone()) {
            None => return Ok(self),
            Some(v) => v,
        };

        let mut skip = false;
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--short" | "-s" | "--porcelain" => {
                    self.git.set_subcommand(GitCommand::Status(false))
                }
                _ => (),
            }
            match (skip, range(&arg)) {
                (false, Some([s, e])) => self.load_range(s, e),
                _ => self.git.arg_unchecked(&arg),
            }
            skip = git_command.skip_next_arg(&arg);
        }
        self.git.args_unchecked(args);

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    macro_rules! test {
        ($name:ident, $args:expr, $test_fn:expr) => {
            test!($name, std::env::temp_dir(), $args, $test_fn);
        };
        ($name:ident, $cwd:expr, $args:expr, $test_fn:expr) => {
            #[test]
            fn $name() {
                let mut args = vec![""];
                args.extend($args);
                let args = args.iter().map(|v| v.to_string());
                let app = App::new(PathBuf::from($cwd));
                let test_fn: Box<dyn Fn(App) -> ()> = Box::new($test_fn);
                test_fn(app.parse(args).unwrap());
            }
        };
    }

    macro_rules! assert_args {
        ($received:expr, $expected:expr) => {{
            let expected =
                $expected.iter().map(|v| v.to_string()).collect::<Vec<_>>();
            let received = $received.git.get_string_args();
            assert_eq_pretty!(received, expected);
        }};
    }

    test!(test_status, "/home", ["-C", "/tmp", "status"], |app| {
        assert_eq!(app.git.subcommand(), Some(&GitCommand::Status(true)));
    });

    test!(test_single, ["add", "1"], |app| {
        assert_args!(app, ["add", "1"]);
    });

    test!(test_range, ["add", "2-4"], |app| {
        assert_args!(app, ["add", "2", "3", "4"]);
    });

    test!(test_mix, ["add", "8", "2-4"], |app| {
        assert_args!(app, ["add", "8", "2", "3", "4"]);
    });

    // Gitnu will not seek to interfere with these cases smartly.
    test!(test_overlap, ["add", "3-5", "2-4"], |app| {
        assert_args!(app, ["add", "3", "4", "5", "2", "3", "4"]);
    });

    // anything after `--` will also be processed. This is for commands
    // like `git reset` which requires pathspecs to appear after --.
    test!(test_double_dash, ["add", "3-5", "--", "2-4"], |app| {
        assert_args!(app, ["add", "3", "4", "5", "--", "2", "3", "4"]);
    });

    test!(test_zeroes_1, ["add", "0"], |app| {
        assert_args!(app, ["add", "0"]);
    });

    test!(test_zeroes_2, ["add", "0-1"], |app| {
        assert_args!(app, ["add", "0", "1"]);
    });
    test!(test_zeroes_3, ["add", "0-0"], |app| {
        assert_args!(app, ["add", "0"]);
    });

    // Filenames containing dashed dates
    test!(test_date_filename, ["add", "2021-01-31"], |app| {
        assert_args!(app, ["add", "2021-01-31"]);
    });
}
