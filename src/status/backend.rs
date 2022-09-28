use crate::opts::{Opts, StatusFmt, LIMIT};
use std::io::BufReader;
use std::process::ChildStdout;

#[derive(PartialEq, Debug)]
enum LineState {
    DualStaged, // both staged and unstaged
    Unstaged,
    Staged,
    Untracked,
    None,
}

type LoadedReader = BufReader<ChildStdout>;

/// Get the state of a line of output of `git status --porcelain`
fn get_line_state(line: &str) -> LineState {
    // lines 3 chars or shorter do not have a filename
    if line.len() <= 3 {
        return LineState::None;
    }
    let mut ch = line.chars();
    let a = ch.next().unwrap_or(' ');
    let b = ch.next().unwrap_or(' ');
    match (a, b) {
        (' ', ' ') => LineState::None,
        ('?', '?') => LineState::Untracked,
        (' ', _) => LineState::Unstaged,
        (_, ' ') => LineState::Staged,
        (_, _) => LineState::DualStaged,
    }
}

#[test]
fn test_get_line_state() {
    let tests = [
        ("A  gold", LineState::Staged),
        ("?? silver", LineState::Untracked),
        (" M bronze", LineState::Unstaged),
        (" M", LineState::None),
        ("D ", LineState::None),
        ("??", LineState::None),
        (" M ", LineState::None),
        ("D  ", LineState::None),
        ("?? ", LineState::None),
        ("", LineState::None),
        (" ", LineState::None),
        ("  ", LineState::None),
        ("   ", LineState::None),
        ("    ", LineState::None),
    ];
    tests.iter().for_each(|(r, e)| assert_eq!(get_line_state(r), *e));
}

/// Goes through the output of git status and obtains files in the order
/// which they are displayed.
fn get_files(reader: LoadedReader, limit: usize) -> Vec<String> {
    let vec = || -> Vec<String> { Vec::new() };

    // staged, unstaged, untracked
    let mut staged = vec();
    let mut unstaged = vec();
    let mut untracked = vec();

    use std::io::BufRead;
    let mut it = reader.lines().filter_map(|line| line.ok());

    while let Some(line) = it.next() {
        let file = String::from(&line[3..]);
        use LineState::*;
        match get_line_state(&line) {
            DualStaged => {
                staged.push(file.clone());
                unstaged.push(file);
            }
            Staged => staged.push(file),
            Unstaged => unstaged.push(file),
            Untracked => untracked.push(file),
            _ => (),
        }
        if staged.len() >= limit {
            return staged;
        }
    }

    fn truncate(v: &mut Vec<String>, limit: usize) {
        if v.len() > limit {
            v.truncate(limit);
        }
    }

    // join all vectors to form one
    // this reflects the order shown with `git status`
    staged.append(&mut unstaged);
    truncate(&mut staged, limit);
    staged.append(&mut untracked);
    truncate(&mut staged, limit);
    staged
}

/// same as get_files but for short displays
fn get_files_short(reader: LoadedReader, limit: usize) -> Vec<String> {
    use std::io::BufRead;
    reader
        .lines()
        .filter_map(|line| line.ok())
        .take(limit)
        .map(|v| String::from(&v[3..]))
        .collect()
}

/// use `git status --pocelain` to get all files in order of display
/// as in `git status`
pub fn run(opts: Opts) -> Option<()> {
    use crate::opts::Commands;
    let mut git = opts.cmd()?;
    git.args(["status", "--porcelain"]);
    git.stdout(std::process::Stdio::piped()); // capture stdout

    // spawn the process
    let output = git.spawn().ok()?.stdout?;

    let reader = BufReader::new(output);
    let files = match opts.status_fmt {
        StatusFmt::Normal => get_files(reader, LIMIT),
        StatusFmt::Short => get_files_short(reader, LIMIT),
    };

    use crate::opts::CacheOps;
    opts.write_cache(files.join("\n"))
}
