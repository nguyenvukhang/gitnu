use crate::bench::Bench;
use crate::test::Test;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command, Stdio};

fn cmd() -> Command {
    let mut cmd = Command::new("echo");
    cmd.args(std::iter::repeat("hello").take(1000));
    cmd.stdout(Stdio::piped());
    cmd
}

fn buffered(target: impl Write) -> Option<()> {
    let mut proc = cmd().spawn().ok()?;
    let mut writer = BufWriter::new(target);
    let br = BufReader::new(proc.stdout.take().unwrap());
    br.lines().filter_map(|v| v.ok()).for_each(|v| {
        writeln!(writer, "{}", v).ok();
    });
    proc.wait().ok();
    writer.flush().ok().map(|_| ())
}

fn synchronous(mut target: impl Write) -> Option<()> {
    let mut proc = cmd().spawn().ok()?;
    let br = BufReader::new(proc.stdout.as_mut()?);
    br.lines().filter_map(|v| v.ok()).for_each(|v| {
        writeln!(target, "{}", v).ok();
    });
    proc.wait().ok().map(|_| ())
}

#[allow(dead_code)]
pub fn buffered_vs_sync(runs: u32) {
    let idle = || Test::new();
    let mut buffered_file =
        Bench::new(runs, idle, |t| buffered(t.file("output")));
    let mut buffered_stdout =
        Bench::new(runs, idle, |_| buffered(std::io::stdout()));
    let mut synchronous_file =
        Bench::new(runs, idle, |t| synchronous(t.file("output")));
    let mut synchronous_stdout =
        Bench::new(runs, idle, |_| synchronous(std::io::stdout()));
    buffered_file.run();
    buffered_stdout.run();
    synchronous_file.run();
    synchronous_stdout.run();
    println!("buffered file:{}", buffered_file);
    println!("synchronous file:{}", synchronous_file);
    println!("buffered stdout:{}", buffered_stdout);
    println!("synchronous stdout:{}", synchronous_stdout);
}
