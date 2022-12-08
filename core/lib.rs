use std::io::{BufRead, BufReader, Read};
use std::process::Command;
use std::{fs::File, path::PathBuf};

mod cache;
mod command;
mod error;
mod git;
mod git_cmd;
mod line;
mod parser;
mod status;

pub use error::GitnuError;
pub use parser::parse;

pub(self) use cache::Cache;
pub(self) use error::ToGitnuError;
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
    /// Sets the subcommand of the App.
    pub fn set_subcommand(&mut self, s: Subcommand) {
        match (&self.subcommand, &s) {
            (Unset, _) | (Status(true), Status(false)) => self.subcommand = s,
            _ => (),
        }
    }

    pub fn arg<S: AsRef<std::ffi::OsStr>>(&mut self, arg: S) {
        self.cmd.arg(arg);
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

    pub fn run(&mut self) -> Result<(), GitnuError> {
        use command::CommandOps;
        match self.subcommand {
            Status(is_normal) => status::status(self, is_normal),
            Version => {
                let result = self.cmd.run();
                println!("gitnu version {}", VERSION.unwrap_or("unknown"));
                result
            }
            _ => self.cmd.run(),
        }
    }
}

impl App {
    pub fn mock(args: &[&str], cwd: &str, sc: Subcommand, argc: usize) -> App {
        let mut app = App::new(PathBuf::from(cwd));
        app.argc = app.cmd.get_args().count();
        app.cmd.args(args).current_dir(&app.cwd);
        app.subcommand = sc;
        app.argc += argc;
        app
    }
}

impl PartialEq for App {
    fn eq(&self, b: &Self) -> bool {
        let subcommand = self.subcommand == b.subcommand;
        let cmd = self.cmd.get_args().eq(b.cmd.get_args())
            && self.cmd.get_current_dir().eq(&b.cmd.get_current_dir());
        subcommand && cmd && self.cwd.eq(&b.cwd) && self.argc == b.argc
    }
}

/// Conveniently converts either a `File` or `Output` into an iterator of
/// `String`s, dropping the invalid ones.
fn lines<R: Read>(src: R) -> impl Iterator<Item = String> {
    BufReader::new(src).lines().filter_map(|v| v.ok())
}
