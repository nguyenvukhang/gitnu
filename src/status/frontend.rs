use crate::opts::Opts;

fn has_color(s: &str) -> bool {
    s.contains("\x1b[31m") || s.contains("\x1b[32m")
}

fn print_line(line: &String, count: &mut u16, limit: u16) {
    if has_color(&line) {
        *count += 1;
        if *count > limit {
            return;
        }
        if line.starts_with('\t') {
            println!("{}{}", count, line);
        } else {
            println!("{: <4}{}", count, line);
        }
    } else {
        println!("{}", line);
    }
}

// this prints `git status` enumerated
// has nothing to do with data management
pub fn run<'a>(opts: Opts) -> Result<(), &'a str> {
    let mut git = match opts.cmd() {
        Ok(v) => v,
        Err(_) => return Err("frontend::run() -> no git command found"),
    };
    git.args(["-c", "status.color=always", "status"]);
    git.stdout(std::process::Stdio::piped()); // capture stdout

    // spawn the process
    let mut git = match git.spawn() {
        Ok(v) => v,
        Err(_) => return Err("frontend::run() -> no output"),
    };

    // get stdout
    let output = match git.stdout.as_mut() {
        Some(v) => v,
        None => return Err("frontend::run() -> no output"),
    };

    // start couting
    let mut count = 0;
    let limit = 50;

    use std::io::BufRead;
    std::io::BufReader::new(output)
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| print_line(&line, &mut count, limit));

    if count > limit {
        println!("... {} hidden items (gitnu)", count - limit);
    }

    match git.wait() {
        Ok(_) => (),
        Err(_) => (),
    };
    return Ok(());
}
