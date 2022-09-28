use crate::opts::{Opts, LIMIT};
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

fn truncate(v: &mut Vec<String>, limit: usize) {
    if v.len() > limit {
        v.truncate(limit);
    }
}

fn get_line_state(line: &str) -> LineState {
    // lines that are 3 chars or shorter do not have a filename
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
    assert_eq!(get_line_state("A  gold"), LineState::Staged);
    assert_eq!(get_line_state("?? silver"), LineState::Untracked);
    assert_eq!(get_line_state(" M bronze"), LineState::Unstaged);
    // no filenames
    assert_eq!(get_line_state(" M"), LineState::None);
    assert_eq!(get_line_state("D "), LineState::None);
    assert_eq!(get_line_state("??"), LineState::None);
    assert_eq!(get_line_state(" M "), LineState::None);
    assert_eq!(get_line_state("D  "), LineState::None);
    assert_eq!(get_line_state("?? "), LineState::None);
    // whitespaces
    assert_eq!(get_line_state(""), LineState::None);
    assert_eq!(get_line_state(" "), LineState::None);
    assert_eq!(get_line_state("  "), LineState::None);
    assert_eq!(get_line_state("   "), LineState::None);
    assert_eq!(get_line_state("    "), LineState::None);
}

fn get_files(reader: BufReader<ChildStdout>, limit: usize) -> Vec<String> {
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

    // join all vectors to form one
    // this reflects the order shown with `git status`
    staged.append(&mut unstaged);
    truncate(&mut staged, limit);
    staged.append(&mut untracked);
    truncate(&mut staged, limit);
    staged
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

    let files = get_files(BufReader::new(output), LIMIT);

    use crate::opts::CacheActions;
    opts.write_cache(files.join("\n"))
}
