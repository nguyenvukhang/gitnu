#![cfg(test)]
use crate::{load, parse, Op, Opts};
use std::path::Path;

#[test]
fn unit_tests() {
    std::env::set_current_dir(Path::new("/")).ok();
    struct Test<'a> {
        received: &'a [&'a str],
        expected: &'a [&'a str],
        cwd: &'a str,
        op: Op,
        cache: Vec<&'a str>,
    }

    fn check(t: Test) {
        // get received args/opts
        let ts = |v: &&str| String::from(*v);
        let rc = [&["gitnu"], t.received].concat();
        let rc = parse(rc.iter().map(|v| v.to_string()));
        let mut cache = vec!["0"];
        cache.extend(t.cache);
        let rc = (load(rc.0, cache.iter().map(ts).collect()), rc.1);
        // get expected opts/args
        let ex = Opts { op: t.op, cwd: t.cwd.into() };
        let ex_args: Vec<String> = t.expected.iter().map(ts).collect();
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
        expected: &["-C", "/dev/null"],
        cwd: "/dev/null",
        op: Op::Number("git".into()),
        cache: vec![],
    });

    // use the value of -x as the command
    check(Test {
        received: &["-x", "nvim"],
        expected: &[],
        cwd: "",
        op: Op::Number("nvim".into()),
        cache: vec![],
    });

    // set both cwd and xargs_cmd
    check(Test {
        received: &["-C", "/etc", "-x", "nvim"],
        expected: &["-C", "/etc"],
        cwd: "/etc",
        op: Op::Number("nvim".into()),
        cache: vec![],
    });

    // set cwd after xargs_cmd
    check(Test {
        received: &["-x", "nvim", "-C", "/etc"],
        expected: &["-C", "/etc"],
        cwd: "",
        op: Op::Number("nvim".into()),
        cache: vec![],
    });

    // cwd + xargs_cmd + weird args
    check(Test {
        received: &["-C", "/etc", "-x", "nvim", "status", "--porcelain"],
        expected: &["-C", "/etc", "status", "--porcelain"],
        cwd: "/etc",
        op: Op::Number("nvim".into()),
        cache: vec![],
    });

    // status mode: (normal)
    check(Test {
        received: &["status"],
        expected: &["status"],
        cwd: "",
        op: Op::Status(true),
        cache: vec![],
    });

    // status mode: --short
    check(Test {
        received: &["status", "--short"],
        expected: &["status", "--short"],
        cwd: "",
        op: Op::Status(false),
        cache: vec![],
    });

    // status mode: -s
    check(Test {
        received: &["status", "-s"],
        expected: &["status", "-s"],
        cwd: "",
        op: Op::Status(false),
        cache: vec![],
    });

    // status mode: --porcelain
    check(Test {
        received: &["status", "--porcelain"],
        expected: &["status", "--porcelain"],
        cwd: "",
        op: Op::Status(false),
        cache: vec![],
    });

    // read mode + range
    check(Test {
        received: &["ls-files", "2-4"],
        expected: &["ls-files", "2", "3", "4"],
        cwd: "",
        op: Op::Number("git".into()),
        cache: vec![],
    });

    // read mode with cwd
    check(Test {
        received: &["-C", "/tmp", "ls-files", "2-4"],
        expected: &["-C", "/tmp", "ls-files", "2", "3", "4"],
        cwd: "/tmp",
        op: Op::Number("git".into()),
        cache: vec![],
    });

    // -C flag without value
    check(Test {
        received: &["-C"],
        expected: &["-C"],
        cwd: "",
        op: Op::Number("git".into()),
        cache: vec![],
    });

    // -x flag without value
    check(Test {
        received: &["-x"],
        expected: &["-x"],
        cwd: "",
        op: Op::Number("git".into()),
        cache: vec![],
    });

    // bypass anything after a short flag
    check(Test {
        received: &["log", "-n", "4", "--oneline", "2-4"],
        expected: &["log", "-n", "4", "--oneline", "2", "3", "4"],
        cwd: "",
        op: Op::Number("git".into()),
        cache: vec![],
    });

    // xargs + range
    check(Test {
        received: &["-x", "nvim", "2-4"],
        expected: &["2", "3", "4"],
        cwd: "",
        op: Op::Number("nvim".into()),
        cache: vec![],
    });

    // repeated numbers
    check(Test {
        received: &["ls-files", "1", "1", "2"],
        expected: &["ls-files", "gold", "gold", "silver"],
        cwd: "",
        op: Op::Number("git".into()),
        cache: vec!["gold", "silver"],
    });
}
