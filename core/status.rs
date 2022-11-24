use crate::{lines, App};
use std::io::{LineWriter, Write};
use std::path::PathBuf;
use std::process::Stdio;

struct State {
    seen_untracked: bool,
    count: usize,
}

fn uncolor(base: &str) -> Vec<u8> {
    let mut base = base.as_bytes();
    let mut dst: Vec<u8> = vec![];
    while !base.is_empty() {
        match base.iter().position(|&b| b == b'\x1b') {
            None => break,
            Some(i) => {
                dst.extend(&base[..i]);
                base = &base[i..];
            }
        }
        match base.iter().position(|&b| b == b'm') {
            None => break,
            Some(i) => base = &base[i + 1..],
        }
    }
    dst.extend(base);
    dst
}

fn pust_past_all(base: &mut &[u8], ch: u8) {
    while !base.is_empty() {
        *base = match base.iter().position(|&b| b == ch) {
            None => break,
            Some(i) => &base[i + 1..],
        };
    }
}

fn pust_past_next(base: &mut &[u8], ch: u8) {
    if let Some(i) = base.iter().position(|&b| b == ch) {
        *base = &base[i + 1..]
    };
}

fn trim_left(base: &mut &[u8], ch: u8) {
    while !base.is_empty() {
        *base = match base[0] {
            v if v == ch => &base[1..],
            _ => break,
        };
    }
}

fn normal(state: &mut State, app: &App, line: String) -> Option<PathBuf> {
    state.seen_untracked |= line.contains("Untracked files:");
    if !line.starts_with('\t') {
        println!("{}", line);
        return None;
    }
    println!("{}{}", state.count, line);
    state.count += 1;
    let mut line: &[u8] = &uncolor(&line);
    let ptr = &mut line;
    pust_past_all(ptr, b'\t');
    let is_rename = ptr.starts_with(b"renamed:");
    if !state.seen_untracked {
        pust_past_all(ptr, b':');
    }
    trim_left(ptr, b' ');
    if is_rename {
        pust_past_next(ptr, b' ');
        pust_past_next(ptr, b' ');
    }
    if line.is_empty() {
        return None;
    }
    std::str::from_utf8(line).ok().map(|v| app.cwd.join(v))
}

fn short(state: &mut State, app: &App, line: String) -> PathBuf {
    println!("{: <3}{}", state.count, line);
    state.count += 1;
    let line = String::from_utf8(uncolor(&line)).unwrap();
    app.cwd.join(&line[3..])
}

fn inner(app: &mut App, is_normal: bool) -> Option<()> {
    let mut git = app.cmd.stdout(Stdio::piped()).spawn().ok()?;
    let lines = lines(git.stdout.as_mut()?);
    let writer = &mut app.cache(true).map(LineWriter::new);
    let write = |line: PathBuf| {
        writer.as_mut().map(|lw| writeln!(lw, "{}", line.display()));
    };
    let state = &mut State { seen_untracked: false, count: 1 };
    if is_normal {
        lines.filter_map(|v| normal(state, &app, v)).for_each(write);
    } else {
        lines.map(|v| short(state, &app, v)).for_each(write);
    };
    git.wait().ok();
    writer.as_mut().map(|lw| lw.flush().ok()).flatten();
    Some(())
}

/// Endpoint function for everything git-status related.
///
/// Runs `git status` then parses its output, enumerates it, and
/// prints it out to stdout.
pub fn status(app: &mut App, is_normal: bool) {
    inner(app, is_normal);
}
