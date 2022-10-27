use crate::{lines, Opts};
use std::io::{LineWriter, Write};
use std::process::Stdio;

pub fn status(mut o: Opts, is_normal: bool) -> Option<()> {
    const C: [&str; 3] = ["\x1b[31m", "\x1b[32m", "\x1b[m"];
    let rmc = |v: &str| v.replace(C[0], "").replace(C[1], "").replace(C[2], "");
    let mut count = 1;
    let mut su = false;
    o.cmd.stdout(Stdio::piped());
    let mut git = o.cmd.spawn().ok()?;
    let b = lines(git.stdout.as_mut()?);
    let mut writer = o.cache().map(LineWriter::new);
    b.filter_map(|line| {
        su |= line.contains("Untracked files:");
        match (is_normal, line.starts_with('\t')) {
            (true, false) => {
                println!("{}", line);
                None
            }
            (true, true) => {
                println!("{}{}", count, line);
                count += 1;
                let f = rmc(&line.trim_start_matches('\t'));
                let f = if su { &f } else { f.split_once(':')?.1.trim_start() };
                Some(o.cwd.join(f))
            }
            _ => {
                println!("{: <3}{}", count, line);
                count += 1;
                Some(o.cwd.join(&rmc(&line)[3..]))
            }
        }
    })
    .for_each(|line| {
        writer.as_mut().map(|lw| writeln!(lw, "{}", line.display()));
    });
    git.wait().ok();
    writer.map(|mut lw| lw.flush().ok()).flatten()
}
