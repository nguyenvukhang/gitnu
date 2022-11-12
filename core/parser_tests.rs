#[cfg(test)]
mod tests {
    use crate::test;
    use crate::{parser, Op, Opts};
    use std::path::PathBuf;

    pub fn parse(args: &[&str], path: &str) -> Opts {
        parser::parse(
            test::iter([&["gitnu"], args].concat()),
            PathBuf::from(path),
        )
    }

    fn setup() {
        std::env::set_current_dir("/tmp").ok();
    }

    const CSA: &str = "color.status=always";

    #[test]
    fn parse_no_ops() {
        setup();
        // no-ops
        assert_eq!(
            parse(&["-C", "/tmp"], "/home"),
            test::opts(
                ("git", &["-C", "/tmp"]),
                ("/tmp", Op::Unset),
                &["-C", "/tmp"]
            ),
        );
    }

    #[test]
    fn parse_status() {
        setup();
        // gitnu <options> status
        assert_eq!(
            parse(&["status"], "/tmp"),
            test::opts(
                ("git", &["-c", CSA, "status"]),
                ("/tmp", Op::Status(true)),
                &[]
            ),
        );
        assert_eq!(
            parse(&["-C", "/tmp", "status"], "/home"),
            test::opts(
                ("git", &["-C", "/tmp", "-c", CSA, "status"]),
                ("/tmp", Op::Status(true)),
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
            test::opts(("git", &["add", "1"]), ("/home", Op::Number), &[]),
        );
        assert_eq!(
            parse(&["add", "2-4"], "/home"),
            test::opts(
                ("git", &["add", "2", "3", "4"]),
                ("/home", Op::Number),
                &[]
            ),
        );
        assert_eq!(
            parse(&["add", "8", "2-4"], "/home"),
            test::opts(
                ("git", &["add", "8", "2", "3", "4"]),
                ("/home", Op::Number),
                &[]
            ),
        );
        assert_eq!(
            parse(&["add", "8", "--", "2-4"], "/home"),
            test::opts(
                ("git", &["add", "8", "--", "2-4"]),
                ("/home", Op::Number),
                &[]
            ),
        );
    }

    #[test]
    fn parse_general() {
        setup();
        // all together
        assert_eq!(
            parse(&["-C", "/tmp", "add", "2-5"], "/home"),
            test::opts(
                ("git", &["-C", "/tmp", "add", "2", "3", "4", "5"]),
                ("/tmp", Op::Number),
                &["-C", "/tmp"]
            ),
        );
    }

    #[test]
    fn parse_ignore_nums_after_short_flags() {
        setup();
        assert_eq!(
            parse(&["log", "-n", "10", "1-3"], "/home"),
            test::opts(
                ("git", &["log", "-n", "10", "1", "2", "3"]),
                ("/home", Op::Number),
                &[]
            ),
        );
    }
}
