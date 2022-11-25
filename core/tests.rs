use crate::{parser, App, Subcommand};
use std::path::PathBuf;
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
        let args = iter::once(&"git").chain($args.iter());
        parser::parse(args.map(|v| v.to_string()), PathBuf::from($cwd))
    }};
}

/// for quickly generating expected values
macro_rules! app {
    ($args:expr, $cwd:expr, $sc:expr, $argc:expr) => {
        quick_app(&$args, $cwd, $sc, $argc)
    };
}

fn quick_app(args: &[&str], cwd: &str, sc: Subcommand, argc: usize) -> App {
    let mut app = App::new(PathBuf::from(cwd));
    app.cmd.args(args).current_dir(&app.cwd);
    app.subcommand = sc;
    app.argc = argc;
    app
}

impl PartialEq for App {
    fn eq(&self, b: &Self) -> bool {
        let subcommand = self.subcommand == b.subcommand;
        let cmd = self.cmd.get_args().eq(b.cmd.get_args())
            && self.cmd.get_current_dir().eq(&b.cmd.get_current_dir());
        subcommand && cmd && self.cwd.eq(&b.cwd) && self.argc == b.argc
    }
}

#[test]
fn partial_eq_works() {
    // different args
    assert_ne!(app!(["a"], "", Unset, 0), app!(["b"], "", Unset, 0));
    // different path
    assert_ne!(app!([""], "a", Unset, 0), app!([""], "b", Unset, 0));
    // different subcommand
    assert_ne!(app!([""], "", Unset, 0), app!([""], "", Number, 0));
    // different argc count
    assert_ne!(app!([""], "", Unset, 0), app!([""], "", Number, 1));
}

test!(
    parse_no_ops,
    parse!(["-C", "/tmp"], "/home"),
    app!(["-C", "/tmp"], "/tmp", Unset, 2)
);

test!(
    parse_status,
    parse!(["status"], "/tmp"),
    app!(["status"], "/tmp", Status(true), 0)
);

test!(
    parse_status_diff_dir,
    parse!(["-C", "/tmp", "status"], "/home"),
    app!(["-C", "/tmp", "status"], "/tmp", Status(true), 2)
);

test!(
    parse_enumerate_single,
    parse!(["add", "1"], "/home"),
    app!(["add", "1"], "/home", Number, 0)
);

test!(
    parse_enumerate_range,
    parse!(["add", "2-4"], "/home"),
    app!(["add", "2", "3", "4"], "/home", Number, 0)
);

test!(
    parse_enumerate_mix,
    parse!(["add", "8", "2-4"], "/home"),
    app!(["add", "8", "2", "3", "4"], "/home", Number, 0)
);

test!(
    parse_enumerate_overlap,
    parse!(["add", "3", "2-4"], "/home"),
    app!(["add", "3", "2", "3", "4"], "/home", Number, 0)
);

test!(
    parse_enumerate_double_dash,
    parse!(["add", "8", "--", "2-4"], "/home"),
    app!(["add", "8", "--", "2-4"], "/home", Number, 0)
);

test!(
    parse_general,
    parse!(["-C", "/tmp", "add", "2-4"], "/home"),
    app!(["-C", "/tmp", "add", "2", "3", "4"], "/tmp", Number, 2)
);

test!(
    parse_version,
    parse!(["--version"], "/home"),
    app!(["--version"], "/home", Version, 0)
);

test!(
    parse_version_weird,
    parse!(["--version", "status"], "/home"),
    app!(["--version", "status"], "/home", Version, 0)
);
