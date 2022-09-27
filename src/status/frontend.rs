use crate::opts::{Opts, LIMIT};
use std::io::Error;

fn has_color(s: &str) -> bool {
    s.contains("\x1b[31m") || s.contains("\x1b[32m")
}

fn print_line(line: &String, count: &mut usize) {
    // only work with colored lines
    if !has_color(&line) {
        println!("{}", line);
        return;
    }
    // line is colored
    *count += 1;
    if *count > LIMIT {
        return;
    }
    if line.starts_with('\t') {
        println!("{}{}", count, line);
    } else {
        println!("{: <4}{}", count, line);
    }
}

// this prints `git status` enumerated
// has nothing to do with data management
pub fn run(opts: Opts) -> Result<(), Error> {
    let mut git = opts.cmd()?;
    git.args(["-c", "status.color=always", "status"]);
    git.stdout(std::process::Stdio::piped()); // capture stdout

    // spawn the process
    let mut git = git.spawn()?;

    // get stdout
    let output = git.stdout.as_mut().ok_or(Error::new(
        std::io::ErrorKind::NotFound,
        "Unable to get stdout",
    ))?;

    // start couting
    let mut count = 0;

    use std::io::{BufRead, BufReader};
    BufReader::new(output)
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| print_line(&line, &mut count));

    if count > LIMIT {
        println!("... {} hidden items (gitnu)", count - LIMIT);
    }

    git.wait().map(|_| ())
}
