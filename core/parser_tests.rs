macro_rules! assert_eq_pretty {
    ($received:expr, $expected:expr) => {
        let expected = $expected;
        let received = $received;
        if expected != received {
            panic!(
                "
printed outputs differ!

~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
expected:
{:?}

received:
{:?}
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
",
                expected, received
            );
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::{parser, App, Subcommand};
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;
    use Subcommand::*;

    impl PartialEq for App {
        fn eq(&self, other: &Self) -> bool {
            let subcommand = self.subcommand == other.subcommand;
            let cwd = self.cwd.eq(&other.cwd);
            let pargs = *self.pargs == *other.pargs;
            let cmd = {
                let (a, b) = (&self.cmd, &other.cmd);
                let args = a.get_args().eq(b.get_args());
                let cwd = {
                    let (a, b) = (a.get_current_dir(), b.get_current_dir());
                    match (a.is_none(), b.is_none()) {
                        (false, false) => *a.unwrap() == *b.unwrap(),
                        (true, true) => true,
                        _ => false,
                    }
                };
                args && cwd
            };
            subcommand && cwd && cmd && pargs
        }
    }

    /// Set expected value
    fn app(
        args: &[&str],
        cwd: &str,
        subcommand: Subcommand,
        pargs: &[&str],
    ) -> App {
        let pargs = pargs.iter().map(|v| v.to_string()).collect();
        let mut cmd = Command::new("");
        if atty::is(atty::Stream::Stdout) {
            cmd.args(["-c", "color.ui=always"]);
        }
        cmd.args(args);
        cmd.current_dir(cwd);
        App { pargs, subcommand, cmd, cwd: PathBuf::from(cwd) }
    }

    /// Get received value
    fn parse(args: &[&str], path: &str) -> App {
        let st = |v: &&str| String::from(*v);
        let args = std::iter::once("".to_string()).chain(args.iter().map(st));
        parser::parse(args, PathBuf::from(path))
    }

    fn setup() {
        env::set_current_dir(env::temp_dir()).ok();
    }

    #[test]
    fn parse_no_ops() {
        setup();
        // no-ops
        assert_eq_pretty!(
            parse(&["-C", "/tmp"], "/home"),
            app(&["-C", "/tmp"], "/tmp", Unset, &["-C", "/tmp"])
        );
    }

    #[test]
    fn parse_status() {
        setup();
        // gitnu <options> status
        assert_eq_pretty!(
            parse(&["status"], "/tmp"),
            app(&["status"], "/tmp", Status(true), &[])
        );
        assert_eq_pretty!(
            parse(&["-C", "/tmp", "status"], "/home"),
            app(
                &["-C", "/tmp", "status"],
                "/tmp",
                Status(true),
                &["-C", "/tmp"]
            )
        );
    }

    #[test]
    fn parse_enumerate() {
        setup();
        // gitnu <command> <numbers>
        assert_eq_pretty!(
            parse(&["add", "1"], "/home"),
            app(&["add", "1"], "/home", Number, &[])
        );
        assert_eq_pretty!(
            parse(&["add", "2-4"], "/home"),
            app(&["add", "2", "3", "4"], "/home", Number, &[])
        );
        assert_eq_pretty!(
            parse(&["add", "8", "2-4"], "/home"),
            app(&["add", "8", "2", "3", "4"], "/home", Number, &[])
        );
        assert_eq_pretty!(
            parse(&["add", "8", "--", "2-4"], "/home"),
            app(&["add", "8", "--", "2-4"], "/home", Number, &[])
        );
    }

    #[test]
    fn parse_general() {
        setup();
        let args = ["-C", "/tmp", "add", "2", "3", "4", "5"];
        // all together
        assert_eq_pretty!(
            parse(&["-C", "/tmp", "add", "2-5"], "/home"),
            app(&args, "/tmp", Number, &["-C", "/tmp"])
        );
    }

    #[test]
    fn parse_ignore_nums_after_short_flags() {
        setup();
        let args = ["log", "-n", "10", "1", "2", "3"];
        assert_eq_pretty!(
            parse(&["log", "-n", "10", "1-3"], "/home"),
            app(&args, "/home", Number, &[])
        );
    }
}
