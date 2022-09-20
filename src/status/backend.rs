// this reads file indices from `git status --porcelain`
// has nothing to do with terminal display/stdout

use crate::opts::Opts;

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

fn get_files(
    reader: std::io::BufReader<std::process::ChildStdout>,
) -> Vec<String> {
    let vec = || -> Vec<String> { Vec::new() };
    let mut staged = vec();
    let mut unstaged = vec();
    let mut untracked = vec();

    // staged, unstaged, untracked
    let handle_line = |line: String| {
        let file = String::from(&line[3..]);
        match get_line_state(&line) {
            LineState::DualStaged => {
                staged.push(file.clone());
                unstaged.push(file);
            }
            LineState::Staged => staged.push(file),
            LineState::Unstaged => unstaged.push(file),
            LineState::Untracked => untracked.push(file),
            _ => (),
        }
    };

    use std::io::BufRead;
    reader.lines().filter_map(|line| line.ok()).for_each(handle_line);

    // join all vectors to form one
    // this reflects the order shown with `git status`
    staged.append(&mut unstaged);
    staged.append(&mut untracked);
    staged
}

/// use `git status --pocelain` to get all files in order of display
/// as in `git status`
pub fn run<'a>(opts: Opts) -> Result<(), &'a str> {
    let mut git = match opts.cmd() {
        Ok(v) => v,
        Err(_) => return Err("backend::run() -> no git command found"),
    };
    git.args(["status", "--porcelain"]);
    git.stdout(std::process::Stdio::piped()); // capture stdout

    // spawn the process
    let git = match git.spawn() {
        Ok(v) => v,
        Err(_) => return Err("backend::run() ->  unable to spawn"),
    };

    // get stdout
    let output = match git.stdout {
        Some(v) => v,
        None => return Err("backend::run() ->  no output"),
    };

    let reader = std::io::BufReader::new(output);
    let files = get_files(reader);

    // write files to json
    let content = files.join("\n");

    match opts.write_cache(content) {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Unable to save to gitnu.txt"),
    };
}
