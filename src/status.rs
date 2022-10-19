use crate::{Op, Opts};
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::{fs::File, path::PathBuf};

fn uncolor(f: &str) -> String {
    f.replace("\x1b[31m", "").replace("\x1b[32m", "").replace("\x1b[m", "")
}

fn print_normal(c: &mut usize, l: &str, o: &Opts, su: bool) -> Option<PathBuf> {
    if !l.starts_with('\t') {
        println!("{}", l);
        return None;
    }
    println!("{}{}", *c, l);
    *c += 1;
    let f = uncolor(l);
    let f = f.split_once('\t')?.1;
    Some(o.cwd.join(if su { f } else { f.split_once(':')?.1.trim_start() }))
}

fn print_short(c: &mut usize, l: &str, o: &Opts) -> Option<PathBuf> {
    println!("{: <3}{}", *c, l);
    *c += 1;
    Some(o.cwd.join(&uncolor(l)[3..]))
}

pub fn run(args: Vec<PathBuf>, opts: Opts) -> Option<()> {
    let mut git = std::process::Command::new("git");
    git.current_dir(&opts.cwd).args(["-c", "color.status=always"]).args(args);
    git.stdout(std::process::Stdio::piped()); // capture stdout
    let (c, mut su, mut git) = (&mut 1, false, git.spawn().ok()?);
    let mut lw = LineWriter::new(File::create(opts.cache_file()?).ok()?);
    let b = BufReader::new(git.stdout.as_mut()?).lines().filter_map(|v| v.ok());
    b.filter_map(|l| {
        su |= l.contains("Untracked files:");
        match opts.op {
            Op::Status(true) => print_normal(c, &l, &opts, su),
            Op::Status(false) => print_short(c, &l, &opts),
            _ => None,
        }
    })
    .for_each(|l| writeln!(lw, "{}", l.display()).unwrap_or(()));
    lw.flush().ok()
}
