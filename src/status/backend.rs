// this reads file indices from `git status --porcelain`
// has nothing to do with terminal display/stdout

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
    assert_eq!(get_line_state(" M"), LineState::None);
    assert_eq!(get_line_state("D "), LineState::None);
    assert_eq!(get_line_state("??"), LineState::None);
    assert_eq!(get_line_state(" M "), LineState::None);
    assert_eq!(get_line_state("D  "), LineState::None);
    assert_eq!(get_line_state("?? "), LineState::None);
    assert_eq!(get_line_state(""), LineState::None);
    assert_eq!(get_line_state(" "), LineState::None);
    assert_eq!(get_line_state("  "), LineState::None);
    assert_eq!(get_line_state("   "), LineState::None);
    assert_eq!(get_line_state("    "), LineState::None);
}

fn get_files(reader: BufReader<ChildStdout>, limit: usize) -> Vec<String> {
    let vec = || -> Vec<String> { Vec::new() };
    let mut staged = vec();
    let mut unstaged = vec();
    let mut untracked = vec();

    // staged, unstaged, untracked
    let handle_line = |line: String| {
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
    };

    use std::io::BufRead;
    reader.lines().filter_map(|line| line.ok()).for_each(handle_line);

    let truncate = |v: &mut Vec<String>| {
        if v.len() > limit {
            v.truncate(limit);
        }
    };

    // join all vectors to form one
    // this reflects the order shown with `git status`
    staged.append(&mut unstaged);
    truncate(&mut staged);
    staged.append(&mut untracked);
    truncate(&mut staged);
    staged
}

/// use `git status --pocelain` to get all files in order of display
/// as in `git status`
pub fn run(opts: Opts) -> Option<()> {
    let mut git = opts.cmd()?;
    git.args(["status", "--porcelain"]);
    git.stdout(std::process::Stdio::piped()); // capture stdout

    // spawn the process
    let git = git.spawn().ok()?;
    let output = git.stdout?;

    let reader = std::io::BufReader::new(output);
    let files = get_files(reader, LIMIT);

    // write files to json
    let content = files.join("\n");

    use crate::actions::CacheActions;
    opts.write_cache(content)
}
