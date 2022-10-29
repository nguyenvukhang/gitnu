use crate::{lines, Opts};
use std::io::{LineWriter, Write};
use std::path::PathBuf;
use std::process::Stdio;

const ANSI: [&str; 3] = ["\x1b[31m", "\x1b[32m", "\x1b[m"];

fn uncolor(v: &str) -> String {
    v.replace(ANSI[0], "").replace(ANSI[1], "").replace(ANSI[2], "")
}

fn normal(
    seen_untracked: &mut bool,
    c: &mut usize,
    opts: &Opts,
    line: String,
) -> Option<PathBuf> {
    *seen_untracked |= line.contains("Untracked files:");
    if !line.starts_with('\t') {
        println!("{}", line);
        return None;
    }
    println!("{}{}", c, line);
    *c += 1;
    let f = uncolor(&line.trim_start_matches('\t'));
    let f = match *seen_untracked {
        true => &f,
        false => f.split_once(':')?.1.trim_start(),
    };
    Some(opts.cwd.join(f))
}

fn short(count: &mut usize, opts: &Opts, line: String) -> PathBuf {
    println!("{: <3}{}", count, line);
    *count += 1;
    opts.cwd.join(&uncolor(&line)[3..])
}

pub fn status(mut opts: Opts, is_normal: bool) -> Option<()> {
    let mut git = opts.cmd.stdout(Stdio::piped()).spawn().ok()?;
    let b = lines(git.stdout.as_mut()?);
    let mut writer = opts.cache().map(LineWriter::new);
    let write = |line: PathBuf| {
        writer.as_mut().map(|lw| writeln!(lw, "{}", line.display()));
    };
    let count = &mut 1;
    if is_normal {
        let su = &mut false;
        b.filter_map(|v| normal(su, count, &opts, v)).for_each(write);
    } else {
        b.map(|v| short(count, &opts, v)).for_each(write);
    };
    git.wait().ok();
    writer.map(|mut lw| lw.flush().ok()).flatten()
}
