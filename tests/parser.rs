use gitnu::{parse, App, Subcommand};
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
        parse(args.map(|v| v.to_string()), PathBuf::from($cwd))
    }};
}

/// for quickly generating expected values
macro_rules! app {
    ($args:expr, $cwd:expr, $sc:expr, $argc:expr) => {
        App::mock(&$args, $cwd, $sc, $argc)
    };
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
    app!(["--version"], "/home", Version, 0)
);
