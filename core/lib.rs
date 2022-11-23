use std::io::{BufRead, BufReader, Read};
use std::{fs::File, path::PathBuf, process::Command};
mod git;
mod git_cmd;
mod parser;
mod status;
pub use parser::parse;
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

    /// Arguments that came before the subcommand.
    /// Essentially these are Git's options, rather than Git's
    /// subcommand's options.
    pargs: Vec<String>,

    /// Cache that came from latest run of `gitnu status`
    cache: Vec<String>,
}

impl App {
    /// use the path obtained from `git rev-parse --git-dir` to store the cache.
    /// this is usually the .git folder of regular repositories, and somewhere
    /// deeper for worktrees.
    fn cache_path(&self) -> Option<PathBuf> {
        // git.stdout returns the git-dir relative to cwd,
        // so prepend it with current dir
        git::git_dir(&self.pargs).map(|v| self.cwd.join(v).join("gitnu.txt"))
    }

    pub fn cache(&self, create: bool) -> Option<File> {
        self.cache_path().and_then(|v| match create {
            true => File::create(v).ok(),
            false => File::open(v).ok(),
        })
    }

    pub fn read_cache(&mut self) {
        std::mem::swap(&mut self.cache, &mut vec!["0".to_string()]);
        self.cache(false).map(|f| lines(f).for_each(|v| self.cache.push(v)));
    }

    pub fn set_once(&mut self, sc: Subcommand) {
        match (&self.subcommand, &sc) {
            (Unset, _) => self.subcommand = sc,
            (Status(true), Status(false)) => self.subcommand = sc,
            _ => (),
        }
    }

    pub fn push_arg(&mut self, arg: &str) {
        self.cmd.arg(arg);
        if self.subcommand == Unset {
            self.pargs.push(arg.to_string());
        }
    }

    pub fn new(cwd: PathBuf) -> Self {
        let mut cmd = Command::new("git");
        if atty::is(atty::Stream::Stdout) {
            cmd.args(["-c", "color.ui=always"]);
        }
        cmd.current_dir(&cwd);
        Self { cwd, subcommand: Unset, pargs: vec![], cache: vec![], cmd }
    }
}

/// Conveniently converts either a `File` or `Output` into an iterator of
/// `String`s, dropping the invalid ones.
fn lines<R: Read>(src: R) -> impl Iterator<Item = String> {
    BufReader::new(src).lines().filter_map(|v| v.ok())
}

/// Runs the command and doesn't look back.
///
/// Call this after parsing is complete and command is fully loaded
/// with all the correct parameters.
pub fn spawn(mut c: Command) {
    c.spawn().and_then(|mut v| v.wait().map(|_| ())).ok();
}

/// Entry point to Gitnu.
///
/// Gitnu's binary calls this function directly, passing in args and
/// current directory obtained from `std::env`.
pub fn run(app: App) {
    match app.subcommand {
        Status(is_normal) => status::status(app, is_normal),
        Version => {
            spawn(app.cmd);
            println!("gitnu version {}", VERSION.unwrap_or("unknown"));
        }
        _ => spawn(app.cmd),
    };
}

#[cfg(test)]
mod tests;
