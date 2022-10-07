use crate::opts::{get_cmd, Opts, StatusFmt};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, LineWriter, Write};
use std::path::PathBuf;

fn uncolor(line: &str) -> String {
    line.replace("\u{1b}[31m", "")
        .replace("\u{1b}[32m", "")
        .replace("\u{1b}[m", "")
}

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

    fn writeln(&mut self, line: Option<&str>) {
        if let Some(ln) = line {
            let ln = self.opts.cwd.join(&ln);
            self.cache.write_fmt(format_args!("{}\n", ln.display())).ok();
        };
    }

    fn print_nu<F: Fn(usize, &str) -> ()>(&mut self, v: &str, p: F) -> String {
        self.count += 1;
        p(self.count, v);
        uncolor(v)
    }

    fn parse_file<'a>(&self, line: &'a String) -> Option<&'a str> {
        match self.opts.status_fmt {
            StatusFmt::Normal => {
                let mut line = line.split_once('\t')?.1;
                if !self.seen_untracked {
                    line = line.split_once(':')?.1.trim_start();
                }
                Some(line)
            }
            StatusFmt::Short => Some(&line[3..]),
        }
    }

    fn print_default(&mut self, line: &str) {
        if !line.starts_with('\t') {
            println!("{}", line);
            return;
        }
        let line = self.print_nu(line, |c, v| println!("{}{}", c, v));
        self.writeln(self.parse_file(&line));
    }

    fn print_short(&mut self, line: &str) {
        let line = self.print_nu(line, |c, v| println!("{: <3}{}", c, v));
        self.writeln(self.parse_file(&line));
    }

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

pub fn run(args: Vec<PathBuf>, opts: Opts) -> Result<(), Error> {
    let err = || Error::new(ErrorKind::Other, "gitnu run failed");
    let mut git = get_cmd(&opts).ok_or(err())?;
    git.args(["-c", "color.status=always"]).args(args);
    git.stdout(std::process::Stdio::piped()); // capture stdout
    let mut git = git.spawn()?;
    let mut printer = {
        let f = File::create(opts.cache_file().ok_or(err())?)?;
        Printer::new(opts, LineWriter::new(f))
    };
    let br = BufReader::new(git.stdout.as_mut().ok_or(err())?);
    br.lines().filter_map(|v| v.ok()).for_each(|v| printer.read(&v));
    printer.cache.flush().ok();
    git.wait().map(|_| ())
}
