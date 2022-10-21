#![cfg(test)]
use crate::{load, parse, Op, Opts};
use std::path::{Path, PathBuf};

#[test]
fn unit_tests() {
    std::env::set_current_dir(Path::new("/")).ok();
    struct Test<'a> {
        received: &'a [&'a str],
        expected: &'a [&'a str],
        cwd: &'a str,
        op: Op,
    }

    fn check(t: Test) {
        // get received args/opts
        let rc = [&["gitnu"], t.received].concat();
        let rc = parse(rc.iter().map(|v| v.to_string()).collect());
        let rc = (load(rc.0, &rc.1), rc.1);
        // get expected opts/args
        let ex = Opts { op: t.op, cwd: t.cwd.into(), gcs: true };
        let ex_args: Vec<PathBuf> =
            t.expected.iter().map(PathBuf::from).collect();
        let ex = (ex_args, ex);
        // at this point, both rc and ex are (args, opts) tuples
        assert_eq!(rc.0, ex.0);
        assert_eq!(rc.1.op, ex.1.op);
        if !t.cwd.eq("") {
            assert_eq!(rc.1.cwd, ex.1.cwd);
        }
    }

    // swallow -C and its value and use it as current dir
    check(Test {
        received: &["-C", "/dev/null"],
        expected: &[],
        cwd: "/dev/null",
        op: Op::Number("git".into()),
    });

    // use the value of -x as the command
    check(Test {
        received: &["-x", "nvim"],
        expected: &[],
        cwd: "",
        op: Op::Number("nvim".into()),
    });

    // set both cwd and xargs_cmd
    check(Test {
        received: &["-C", "/etc", "-x", "nvim"],
        expected: &[],
        cwd: "/etc",
        op: Op::Number("nvim".into()),
    });

    // set cwd after xargs_cmd
    check(Test {
        received: &["-x", "nvim", "-C", "/etc"],
        expected: &["-C", "/etc"],
        cwd: "",
        op: Op::Number("nvim".into()),
    });

    // cwd + xargs_cmd + weird args
    check(Test {
        received: &["-C", "/etc", "-x", "nvim", "status", "--porcelain"],
        expected: &["status", "--porcelain"],
        cwd: "/etc",
        op: Op::Number("nvim".into()),
    });

    // status mode: (normal)
    check(Test {
        received: &["status"],
        expected: &["status"],
        cwd: "",
        op: Op::Status(true),
    });

    // status mode: --short
    check(Test {
        received: &["status", "--short"],
        expected: &["status", "--short"],
        cwd: "",
        op: Op::Status(false),
    });

    // status mode: -s
    check(Test {
        received: &["status", "-s"],
        expected: &["status", "-s"],
        cwd: "",
        op: Op::Status(false),
    });

    // status mode: --porcelain
    check(Test {
        received: &["status", "--porcelain"],
        expected: &["status", "--porcelain"],
        cwd: "",
        op: Op::Status(false),
    });

    // read mode + range
    check(Test {
        received: &["ls-files", "2-4"],
        expected: &["ls-files", "2", "3", "4"],
        cwd: "",
        op: Op::Number("git".into()),
    });

    // read mode with cwd
    check(Test {
        received: &["-C", "/tmp", "ls-files", "2-4"],
        expected: &["ls-files", "2", "3", "4"],
        cwd: "/tmp",
        op: Op::Number("git".into()),
    });

    // -C flag without value
    check(Test {
        received: &["-C"],
        expected: &["-C"],
        cwd: "",
        op: Op::Number("git".into()),
    });

    // -x flag without value
    check(Test {
        received: &["-x"],
        expected: &["-x"],
        cwd: "",
        op: Op::Number("git".into()),
    });

    // bypass anything after a short flag
    check(Test {
        received: &["log", "-n", "4", "--oneline", "2-4"],
        expected: &["log", "-n", "4", "--oneline", "2", "3", "4"],
        cwd: "",
        op: Op::Number("git".into()),
    });

    // xargs + range
    check(Test {
        received: &["-x", "nvim", "2-4"],
        expected: &["2", "3", "4"],
        cwd: "",
        op: Op::Number("nvim".into()),
    });
}
