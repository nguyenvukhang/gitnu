use crate::opts::{Commands, Opts, StatusFmt};
use std::fs::File;
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::PathBuf;

/// Manages the actual output of `gitnu status`
/// Enumerates each line containing a filename up until `limit`
struct Printer {
    count: usize,
    opts: Opts,
    seen_untracked: bool,
    cache: LineWriter<File>,
}

impl Printer {
    pub fn new(opts: Opts, cache: LineWriter<File>) -> Self {
        Self { count: 0, opts, seen_untracked: false, cache }
    }

    /// Removes red and green ANSI codes from a string
    /// Returns a fresh string.
    fn uncolor(line: &str) -> String {
        line.replace("\u{1b}[31m", "")
            .replace("\u{1b}[32m", "")
            .replace("\u{1b}[m", "")
    }

    fn write_cache(&mut self, line: &str) {
        self.cache
            .write_fmt(format_args!("{}\n", self.opts.join(line).display()))
            .ok();
    }

    pub fn flush(&mut self) {
        self.cache.flush().ok();
    }

    /// prints output of `git status`
    fn print_default(&mut self, line: &str) -> Option<()> {
        if !line.starts_with('\t') {
            println!("{}", line);
            return None;
        }

        // print and strip colors
        self.count += 1;
        println!("{}{}", self.count, line);
        let line = Self::uncolor(line);

        // post-print processing
        let mut line = line.split('\t').last()?;
        if !self.seen_untracked {
            line = line.split_once(':')?.1.trim_start();
        }
        Some(self.write_cache(line))
    }

    /// Handles output of `git status --short` or
    /// `git status --porcelain`
    fn print_short(&mut self, line: &str) -> Option<()> {
        // print and strip colors
        self.count += 1;
        println!("{: <3}{}", self.count, line);
        let line = Self::uncolor(line);

        // post-print processing
        Some(self.write_cache(&line[3..]))
    }

    /// Core printing method.
    /// Takes in any line from git output and self-updates count
    pub fn read(&mut self, line: &str) {
        if line.contains("Untracked files:") {
            self.seen_untracked = true;
        }
        match self.opts.status_fmt {
            StatusFmt::Normal => self.print_default(&line),
            StatusFmt::Short => self.print_short(&line),
        };
    }
}

// this prints `git status` enumerated
// has nothing to do with data management
pub fn run(args: Vec<PathBuf>, opts: Opts) -> Option<()> {
    let mut git = opts.cmd()?;
    git.args(["-c", "color.status=always"]).args(args);
    git.stdout(std::process::Stdio::piped()); // capture stdout

    // spawn the process
    let mut git = git.spawn().ok()?;

    let mut p = {
        let cache = File::create(opts.cache_file()?).ok()?;
        Printer::new(opts, LineWriter::new(cache))
    };

    {
        // read stdout and stream the filenames into cache
        let br = BufReader::new(git.stdout.as_mut()?);
        br.lines().filter_map(|v| v.ok()).for_each(|v| p.read(&v));
    }

    p.flush();

    git.wait().map(|_| ()).ok()
}
