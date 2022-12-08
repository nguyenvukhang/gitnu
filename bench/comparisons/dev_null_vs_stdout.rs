use crate::bench::Bench;
use crate::test::Test;
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn run<P: AsRef<std::path::Path>>(cmd: &mut Command, file: P) -> Option<()> {
    let mut t = File::open(file).ok()?;
    let output = cmd.output().ok()?.stdout;
    writeln!(&mut t, "{}", String::from_utf8_lossy(&output)).ok()
}

fn cmd() -> Command {
    let mut cmd = Command::new("echo");
    cmd.args(std::iter::repeat("hello\n").take(1000));
    cmd
}

/// Goal: show that printing to /dev/stdout is slower than /dev/null
/// (which is really obvious in hindsight)
///
/// Output:
///
/// /dev/null:
/// runs:     100
/// std.dev:  134.269µs
/// average:  1.548362ms
/// /dev/stdout:
/// runs:     100
/// std.dev:  305.945µs
/// average:  2.064946ms
/// /tmp/file:
/// runs:     100
/// std.dev:  176.758µs
/// average:  1.563144ms
#[allow(dead_code)]
pub fn dev_null_vs_stdout(runs: u32) {
    let idle = || Test::new();
    let mut dev_null = Bench::new(runs, idle, |_| run(&mut cmd(), "/dev/null"));
    let mut to_stdout =
        Bench::new(runs, idle, |_| run(&mut cmd(), "/dev/stdout"));
    let mut to_file = Bench::new(
        runs,
        || {
            let t = Test::new();
            File::create(t.dir().join("file")).ok();
            t
        },
        |t| run(&mut cmd(), t.dir().join("file")),
    );
    dev_null.run();
    to_stdout.run();
    to_file.run();
    println!("/dev/null:{}", dev_null);
    println!("/dev/stdout:{}", to_stdout);
    println!("/tmp/file:{}", to_file);
}
