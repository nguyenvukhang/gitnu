use crate::structs::Gitnu;
use log::{info};
use std::collections::LinkedList;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};

enum LineState {
    DualStaged, // both staged and unstaged
    Unstaged,
    Staged,
    Untracked,
    None,
}

fn get_line_state(line: &String) -> LineState {
    if line.len() < 2 {
        return LineState::None;
    }
    if line.starts_with("??") {
        return LineState::Untracked;
    };
    let mut it = line.chars();
    let staged = it.next() != Some(' ');
    let unstaged = it.next() != Some(' ');
    match (staged, unstaged) {
        (true, true) => LineState::DualStaged,
        (true, false) => LineState::Staged,
        (false, true) => LineState::Unstaged,
        (false, false) => LineState::None,
    }
}

/// use `git status --pocelain` to get all files in order of display
/// as in `git status`
///
/// index starts at the front of the linked list
pub fn load_files(gitnu: &mut Gitnu, git_dir: &PathBuf) {
    let mut staged: LinkedList<String> = LinkedList::new();
    let mut unstaged: LinkedList<String> = LinkedList::new();
    let mut untracked: LinkedList<String> = LinkedList::new();

    let handle_line = |line: String| {
        let file = line[3..].to_string();
        match get_line_state(&line) {
            LineState::DualStaged => {
                unstaged.push_back(file.clone());
                staged.push_back(file);
            }
            LineState::Staged => staged.push_back(file),
            LineState::Unstaged => unstaged.push_back(file),
            LineState::Untracked => untracked.push_back(file),
            _ => (),
        }
    };

    info!("loading index at: {:#?}", git_dir);

    // spawn the git status process
    let proc = Command::new("git")
        .arg("-C")
        .arg(git_dir)
        .args(["status", "--porcelain"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    // return a reader for that process
    // this enables real-time processing of stdout
    BufReader::new(proc.stdout.unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .for_each(handle_line);

    // after this point, the reader has done executing and has closed

    let mut load = |mut q: LinkedList<String>| {
        while !q.is_empty() {
            gitnu.add_file(q.pop_front().unwrap());
        }
    };

    // load all files into gitnu queue
    load(staged);
    load(unstaged);
    load(untracked);
}
