use crate::{lines, Opts};
use std::io::{LineWriter, Write};
use std::path::PathBuf;
use std::process::Stdio;

const ANSI: [&str; 3] = ["\x1b[31m", "\x1b[32m", "\x1b[m"];

fn uncolor(v: &str) -> String {
    v.replace(ANSI[0], "").replace(ANSI[1], "").replace(ANSI[2], "")
}

fn normal(
    su: &mut bool,
    c: &mut usize,
    opts: &Opts,
    line: String,
) -> Option<PathBuf> {
    *su |= line.contains("Untracked files:");
    match line.starts_with('\t') {
        false => {
            println!("{}", line);
            None
        }
        true => {
            println!("{}{}", c, line);
            *c += 1;
            let f = uncolor(&line.trim_start_matches('\t'));
            let f = if *su { &f } else { f.split_once(':')?.1.trim_start() };
            Some(opts.cwd.join(f))
        }
    }
}

fn short(count: &mut usize, opts: &Opts, line: String) -> PathBuf {
    println!("{: <3}{}", count, line);
    *count += 1;
    opts.cwd.join(&uncolor(&line)[3..])
}

pub fn status(mut opts: Opts, is_normal: bool) -> Option<()> {
    let mut git = opts.cmd.stdout(Stdio::piped()).spawn().ok()?;
    let b = lines(git.stdout.as_mut()?);
    let mut count = 1;
    let mut su = false;
    let mut writer = opts.cache().map(LineWriter::new);
    let write = |line: PathBuf| {
        writer.as_mut().map(|lw| writeln!(lw, "{}", line.display()));
    };
    if is_normal {
        b.filter_map(|v| normal(&mut su, &mut count, &opts, v)).for_each(write);
    } else {
        b.map(|v| short(&mut count, &opts, v)).for_each(write);
    };
    git.wait().ok();
    writer.map(|mut lw| lw.flush().ok()).flatten()
}
