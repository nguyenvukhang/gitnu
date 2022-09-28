use crate::opts::{Opts, StatusFmt, LIMIT};
use std::path::PathBuf;

struct Print {
    count: usize,
    limit: usize,
    fmt: StatusFmt,
}

impl Print {
    pub fn new(limit: usize, opts: Opts) -> Self {
        Print { count: 0, limit, fmt: opts.status_fmt }
    }

    /// prints only if current count is within limits
    fn safe_print<F>(&self, print: F)
    where
        F: FnOnce() -> (),
    {
        if self.count <= self.limit {
            print()
        }
    }

    /// prints output of `git status`
    fn print_default(&mut self, line: &str) {
        if !line.starts_with('\t') {
            println!("{}", line);
            return;
        }
        self.count += 1;
        self.safe_print(|| println!("{}{}", self.count, line));
    }

    /// prints output of `git status --short` or
    /// `git status --porcelain`
    fn print_short(&mut self, line: &str) {
        self.count += 1;
        self.safe_print(|| println!("{: <3}{}", self.count, line));
    }

    /// core printing method
    pub fn print(&mut self, line: &str) {
        match self.fmt {
            StatusFmt::Normal => self.print_default(line),
            _ => self.print_short(line),
        }
    }

    /// tells use how many lines of status was hidden from stdout
    pub fn end(&self) {
        if self.count <= self.limit {
            return;
        }
        let hid = self.count - self.limit;
        println!("... {} hidden items (gitnu)", hid);
    }
}

// this prints `git status` enumerated
// has nothing to do with data management
pub fn run(args: Vec<PathBuf>, opts: Opts) -> Option<()> {
    use crate::opts::Commands;
    let mut git = opts.cmd()?;
    git.args(["-c", "color.status=always"]);
    git.args(args);
    git.stdout(std::process::Stdio::piped()); // capture stdout

    // spawn the process
    let mut git = git.spawn().ok()?;
    let output = git.stdout.as_mut()?;

    use std::io::{BufRead, BufReader};
    let mut printer = Print::new(LIMIT, opts);
    BufReader::new(output)
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| printer.print(&line));
    printer.end();

    git.wait().map(|_| ()).ok()
}
