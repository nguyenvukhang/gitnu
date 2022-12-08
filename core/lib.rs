use std::io::{BufRead, BufReader, Read};
use std::{fs::File, path::PathBuf, process::Command};
mod command;
mod git;
mod git_cmd;
mod line;
mod parser;
mod status;
pub use parser::parse;
use std::io::Lines;
use Subcommand::*;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

/// Git sub-command.
///
/// Lets Gitnu know what output to expect, and whether or not to
/// read/write cache.
#[derive(Debug, PartialEq)]
pub enum Subcommand {
    /// Contained value represents if the status command is the
    /// regular variant.
    ///
    /// `gitnu status` with no flags gives Status(true).
    /// flags `-s`, `--short`, `--porcelain` gives Status(false).
    Status(bool),

    /// Gitnu will fetch cache in this state.
    Number,

    /// A special case where gitnu drops everything and prints its own
    /// version next to git's version.
    Version,

    /// Original state.
    Unset,
}

/// Gitnu's running state.
#[derive(Debug)]
pub struct App {
    /// Controls main flow (read/write/which parser to use)
    subcommand: Subcommand,

    /// Directory that Gitnu was ran from.
    /// This can be overridden by using the `-C` flag, which is
    /// identical behaviour to vanilla Git.
    cwd: PathBuf,

    /// The command that will be ran once all processing is complete.
    cmd: Command,

    /// Numer of arguments that came before the subcommand.
    /// Essentially these are Git's options, rather than Git's
    /// subcommand's options.
    argc: usize,

    /// Cache that came from latest run of `gitnu status`
    cache: Vec<String>,
    buffer: Option<Lines<BufReader<File>>>,
}

impl App {
    /// use the path obtained from `git rev-parse --git-dir` to store the cache.
    /// this is usually the .git folder of regular repositories, and somewhere
    /// deeper for worktrees.
    fn cache_path(&self) -> Option<PathBuf> {
        // git.stdout returns the git-dir relative to cwd,
        // so prepend it with current dir
        git::git_dir(self.cmd.get_args().take(self.argc))
            .map(|v| self.cwd.join(v).join("gitnu.txt"))
    }

    pub fn cache(&self, create: bool) -> Option<File> {
        self.cache_path().and_then(|v| match create {
            true => File::create(v).ok(),
            false => File::open(v).ok(),
        })
    }

    /// eagerly read cache until cache contains the nth file
    pub fn read_until(&mut self, n: usize) {
        let len = self.cache.len();
        if n < len || self.buffer.is_none() {
            return;
        }
        let buffer = self.buffer.as_mut().unwrap().take(n + 1 - len);
        buffer
            .enumerate()
            .map(|(i, v)| v.unwrap_or((len + i).to_string()))
            .for_each(|v| self.cache.push(v));
    }

    /// Adds a file of index n as and argument to the `Command` that
    /// will eventually be run.
    pub fn add_file_by_number(&mut self, n: usize) {
        self.cmd.arg(self.cache.get(n).unwrap_or(&n.to_string()));
    }

    /// Lazily loads cache file into buffer without actually reading
    /// any lines yet.
    pub fn load_cache_buffer(&mut self) {
        self.cache = vec!["0".to_string()];
        self.buffer = self.cache(false).map(|v| BufReader::new(v).lines());
    }

    pub fn set_once(&mut self, s: Subcommand) {
        match (&self.subcommand, &s) {
            (Unset, _) | (Status(true), Status(false)) => self.subcommand = s,
            _ => (),
        }
    }

    pub fn push_arg(&mut self, arg: &str) {
        self.cmd.arg(arg);
        if self.subcommand == Unset {
            self.argc += 1;
        }
    }

    pub fn new(cwd: PathBuf) -> Self {
        let mut cmd = Command::new("git");
        if atty::is(atty::Stream::Stdout) {
            cmd.args(["-c", "color.ui=always"]);
        }
        cmd.current_dir(&cwd);
        Self {
            cwd,
            subcommand: Unset,
            cache: vec![],
            cmd,
            buffer: None,
            argc: 0,
        }
    }

    pub fn run(&mut self) {
        use command::CommandOps;
        use std::process::exit;
        match self.subcommand {
            Status(is_normal) => status::status(self, is_normal),
            Version => {
                let exit_code = self.cmd.run();
                println!("gitnu version {}", VERSION.unwrap_or("unknown"));
                exit(exit_code)
            }
            _ => exit(self.cmd.run()),
        }
    }
}

/// Conveniently converts either a `File` or `Output` into an iterator of
/// `String`s, dropping the invalid ones.
fn lines<R: Read>(src: R) -> impl Iterator<Item = String> {
    BufReader::new(src).lines().filter_map(|v| v.ok())
}

#[cfg(test)]
mod tests;
