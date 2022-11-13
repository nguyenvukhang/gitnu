#[cfg(test)]
mod tests {
    use crate::{parser, Op, Opts};
    use std::env;
    use std::path::PathBuf;
    use std::process::Command;

    impl PartialEq for Opts {
        fn eq(&self, other: &Self) -> bool {
            let op = self.op == other.op;
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
            op && cwd && cmd && pargs
        }
    }

    /// Set expected value
    fn opts(args: &[&str], cwd: &str, op: Op, pargs: &[&str]) -> Opts {
        let pargs = pargs.iter().map(|v| v.to_string()).collect();
        let mut cmd = Command::new("");
        cmd.args(args);
        cmd.current_dir(cwd);
        Opts { pargs, op, cmd, cwd: PathBuf::from(cwd) }
    }

    /// Get received value
    fn parse(args: &[&str], path: &str) -> Opts {
        let st = |v: &&str| String::from(*v);
        let args = std::iter::once("".to_string()).chain(args.iter().map(st));
        parser::parse(args, PathBuf::from(path))
    }

    fn setup() {
        env::set_current_dir(env::temp_dir()).ok();
    }

    const CSA: &str = "color.status=always";

    #[test]
    fn parse_no_ops() {
        setup();
        // no-ops
        assert_eq!(
            parse(&["-C", "/tmp"], "/home"),
            opts(&["-C", "/tmp"], "/tmp", Op::Unset, &["-C", "/tmp"]),
        );
    }

    #[test]
    fn parse_status() {
        setup();
        // gitnu <options> status
        assert_eq!(
            parse(&["status"], "/tmp"),
            opts(&["-c", CSA, "status"], "/tmp", Op::Status(true), &[]),
        );
        assert_eq!(
            parse(&["-C", "/tmp", "status"], "/home"),
            opts(
                &["-C", "/tmp", "-c", CSA, "status"],
                "/tmp",
                Op::Status(true),
                &["-C", "/tmp"]
            ),
        );
    }

    #[test]
    fn parse_enumerate() {
        setup();
        // gitnu <command> <numbers>
        assert_eq!(
            parse(&["add", "1"], "/home"),
            opts(&["add", "1"], "/home", Op::Number, &[]),
        );
        assert_eq!(
            parse(&["add", "2-4"], "/home"),
            opts(&["add", "2", "3", "4"], "/home", Op::Number, &[]),
        );
        assert_eq!(
            parse(&["add", "8", "2-4"], "/home"),
            opts(&["add", "8", "2", "3", "4"], "/home", Op::Number, &[]),
        );
        assert_eq!(
            parse(&["add", "8", "--", "2-4"], "/home"),
            opts(&["add", "8", "--", "2-4"], "/home", Op::Number, &[]),
        );
    }

    #[test]
    fn parse_general() {
        setup();
        // all together
        assert_eq!(
            parse(&["-C", "/tmp", "add", "2-5"], "/home"),
            opts(
                &["-C", "/tmp", "add", "2", "3", "4", "5"],
                "/tmp",
                Op::Number,
                &["-C", "/tmp"]
            ),
        );
    }

    #[test]
    fn parse_ignore_nums_after_short_flags() {
        setup();
        assert_eq!(
            parse(&["log", "-n", "10", "1-3"], "/home"),
            opts(&["log", "-n", "10", "1", "2", "3"], "/home", Op::Number, &[]),
        );
    }
}
