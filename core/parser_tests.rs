#[cfg(test)]
mod tests {
    use crate::{parser, App, Subcommand};
    use std::path::PathBuf;
    use std::process::Command;
    use std::{env, iter};
    use Subcommand::*;

    macro_rules! test {
        ($name:ident, $received:expr, $expected:expr) => {
            #[test]
            fn $name() {
                env::set_current_dir(env::temp_dir()).ok();
                assert_eq!($received, $expected);
            }
        };
    }

    /// for simulating gitnu's parser
    macro_rules! parse {
        ($args:expr, $cwd:expr) => {{
            let args = iter::once("".to_string())
                .chain($args.iter().map(|v| v.to_string()));
            parser::parse(args, PathBuf::from($cwd))
        }};
    }

    /// for quickly generating expected values
    macro_rules! app {
        ($args:expr, $cwd:expr, $sc:expr, $pargs:expr) => {{
            let pargs = $pargs.iter().map(|v: &&str| v.to_string()).collect();
            let mut cmd = Command::new("");
            if atty::is(atty::Stream::Stdout) {
                cmd.args(["-c", "color.ui=always"]);
            }
            cmd.args($args).current_dir($cwd);
            let cwd = PathBuf::from($cwd);
            App { subcommand: $sc, cmd, cwd, pargs, cache: vec![] }
        }};
        ($args:expr, $cwd:expr, $sc:expr) => {{
            let mut cmd = Command::new("");
            if atty::is(atty::Stream::Stdout) {
                cmd.args(["-c", "color.ui=always"]);
            }
            cmd.args($args).current_dir($cwd);
            let cwd = PathBuf::from($cwd);
            App { subcommand: $sc, cmd, cwd, pargs: vec![], cache: vec![] }
        }};
    }

    impl PartialEq for App {
        fn eq(&self, b: &Self) -> bool {
            let subcommand = self.subcommand == b.subcommand;
            let cmd = self.cmd.get_args().eq(b.cmd.get_args())
                && self.cmd.get_current_dir().eq(&b.cmd.get_current_dir());
            subcommand && self.cwd.eq(&b.cwd) && cmd && self.pargs.eq(&b.pargs)
        }
    }

    test!(
        parse_no_ops,
        parse!(["-C", "/tmp"], "/home"),
        app!(["-C", "/tmp"], "/tmp", Unset, ["-C", "/tmp"])
    );

    test!(
        parse_status,
        parse!(["status"], "/tmp"),
        app!(["status"], "/tmp", Status(true))
    );

    test!(
        parse_status_diff_dir,
        parse!(["-C", "/tmp", "status"], "/home"),
        app!(["-C", "/tmp", "status"], "/tmp", Status(true), ["-C", "/tmp"])
    );

    test!(
        parse_enumerate_single,
        parse!(["add", "1"], "/home"),
        app!(["add", "1"], "/home", Number)
    );

    test!(
        parse_enumerate_range,
        parse!(["add", "2-4"], "/home"),
        app!(["add", "2", "3", "4"], "/home", Number)
    );

    test!(
        parse_enumerate_mix,
        parse!(["add", "8", "2-4"], "/home"),
        app!(["add", "8", "2", "3", "4"], "/home", Number)
    );

    test!(
        parse_enumerate_overlap,
        parse!(["add", "3", "2-4"], "/home"),
        app!(["add", "3", "2", "3", "4"], "/home", Number)
    );

    test!(
        parse_enumerate_double_dash,
        parse!(["add", "8", "--", "2-4"], "/home"),
        app!(["add", "8", "--", "2-4"], "/home", Number)
    );

    test!(
        parse_general,
        parse!(["-C", "/tmp", "add", "2-4"], "/home"),
        app!(
            ["-C", "/tmp", "add", "2", "3", "4"],
            "/tmp",
            Number,
            ["-C", "/tmp"]
        )
    );

    test!(
        parse_version,
        parse!(["--version"], "/home"),
        app!(["--version"], "/home", Version)
    );

    test!(
        parse_version_weird,
        parse!(["--version", "status"], "/home"),
        app!(["--version", "status"], "/home", Version)
    );

    #[test]
    fn partial_eq_works() {
        assert_ne!(app!(["a"], "", Unset, [""]), app!(["b"], "", Unset, [""]));
        assert_ne!(app!([""], "a", Unset, [""]), app!([""], "b", Unset, [""]));
        assert_ne!(app!([""], "", Unset, [""]), app!([""], "", Number, [""]));
        assert_ne!(app!([""], "", Unset, ["a"]), app!([""], "", Unset, ["b"]));
    }
}
