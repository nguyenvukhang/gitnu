use std::collections::HashMap;
use std::path::PathBuf;
use std::process::ExitCode;

use crate::git_cmd::GitCommand;
use crate::prelude::*;
use crate::Cache;
use crate::Command2;

type Aliases = HashMap<String, String>;

pub(crate) struct AppBuilder {
    git_aliases: Option<Aliases>,
    git_cmd: Option<GitCommand>,
    git_dir: Option<PathBuf>,
    cwd: PathBuf,
    final_command: Command2,
    cache: Option<Cache>,
}

#[derive(Debug)]
pub(crate) struct App {
    pub git_aliases: Aliases,
    pub git_cmd: Option<GitCommand>,
    pub git_dir: PathBuf,
    pub cwd: PathBuf,
    pub final_command: Command2,
    pub cache: Cache,
}

impl AppBuilder {
    pub fn new(cwd: PathBuf) -> Self {
        Self {
            git_aliases: None,
            git_cmd: None,
            git_dir: None,
            cwd,
            final_command: Command2::new("git"),
            cache: None,
        }
    }
    pub fn build(self) -> App {
        App {
            git_aliases: self.git_aliases.unwrap_or_default(),
            git_cmd: self.git_cmd,
            git_dir: self.git_dir.unwrap_or_default(),
            cwd: self.cwd,
            final_command: self.final_command,
            cache: self.cache.unwrap_or_default(),
        }
    }
}

macro_rules! build {
    ($field:ident, $type:ty) => {
        impl AppBuilder {
            pub fn $field(mut self, v: $type) -> Self {
                self.$field = Some(v);
                self
            }
        }
    };
}

build!(git_aliases, Aliases);
build!(git_dir, PathBuf);
build!(cache, Cache);

impl Default for App {
    fn default() -> Self {
        Self {
            git_aliases: Aliases::new(),
            git_cmd: None,
            git_dir: PathBuf::new(),
            cwd: PathBuf::new(),
            final_command: Command2::new("git"),
            cache: Cache::default(),
        }
    }
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            git_aliases: None,
            git_cmd: None,
            git_dir: None,
            cwd: PathBuf::new(),
            final_command: Command2::new("git"),
            cache: None,
        }
    }
}

/// Parses a string into an inclusive range.
/// "5"   -> Some([5, 5])
/// "2-6" -> Some([2, 6])
/// "foo" -> None
pub fn parse_range(arg: &str) -> Option<(usize, usize)> {
    if let Ok(single) = arg.parse::<usize>() {
        Some((single, single))
    } else {
        let (a, b) = arg.split_once('-')?;
        let a = a.parse::<usize>().ok()?;
        let b = b.parse::<usize>().ok()?;
        Some((a.min(b), a.max(b)))
    }
}

impl App {
    pub fn parse<I>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = String>,
    {
        if atty::is(atty::Stream::Stdout) {
            self.final_command.hidden_args(["-c", "color.ui=always"]);
        }
        let mut args = args.into_iter().skip(1);
        self.before_cmd(&mut args).after_cmd(&mut args);

        self
    }

    fn before_cmd<I>(&mut self, args: &mut I) -> &mut Self
    where
        I: Iterator<Item = String>,
    {
        use GitCommand as GC;
        for arg in args {
            if let Ok(v) = GC::try_from(&arg) {
                self.git_cmd = Some(v);
                self.final_command.arg(arg);
                break;
            }
            if let Some(Ok(v)) = self.git_aliases.get(&arg).map(GC::try_from) {
                self.git_cmd = Some(v);
                self.final_command.arg(arg);
                break;
            }
            self.final_command.arg(arg);
        }
        self
    }

    fn after_cmd<I>(&mut self, args: &mut I) -> &mut Self
    where
        I: Iterator<Item = String>,
    {
        let mut skip = false;

        if let None = self.git_cmd {
            self.final_command.inner.args(args);
            return self;
        }

        for arg in args {
            let arg = arg.as_str();
            let git_cmd = self.git_cmd.as_mut().unwrap();
            match git_cmd {
                GitCommand::Status(ref mut v) => match arg {
                    "--short" | "-s" | "--porcelain" => v.short(),
                    _ => {}
                },
                _ => {}
            };
            match (skip, parse_range(&arg)) {
                (false, Some((start, end))) if end <= MAX_CACHE_SIZE => {
                    let cmd = &mut self.final_command.inner;
                    (start..end + 1).for_each(|i| self.cache.load(i, cmd));
                }
                _ => self.final_command.arg(&arg),
            }
            skip = git_cmd.skip_next_arg(&arg);
        }
        self
    }

    pub(crate) fn run(mut self) -> Result<ExitCode> {
        use GitCommand as G;
        let git_cmd = self.git_cmd.clone();
        match git_cmd {
            Some(G::Version) => {
                let result = self.final_command.inner.status();
                let exitcode = result.map(|v| v.exitcode())?;
                println!("gitnu version {CARGO_PKG_VERSION}");
                Ok(exitcode)
            }
            Some(G::Status(_)) => self.git_status(),
            _ => {
                let result = self.final_command.inner.status();
                let exitcode = result.map(|v| v.exitcode())?;
                Ok(exitcode)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($name:ident, $input_args:expr, $output_args:expr) => {
            #[test]
            fn $name() {
                let mut args = vec!["git"];
                args.extend($input_args);
                let args = args.iter().map(|v| v.to_string());
                let app = App::default().parse(args);
                let received_args = app.final_command.get_args();

                let expected_args: Vec<_> =
                    $output_args.into_iter().map(String::from).collect();
                assert_eq!(received_args, expected_args);
            }
        };
    }

    test!(test_single, ["add", "1"], ["add", "1"]);
    test!(test_range, ["add", "2-4"], ["add", "2", "3", "4"]);
    test!(test_mix, ["add", "8", "2-4"], ["add", "8", "2", "3", "4"]);

    // Gitnu will not seek to interfere with these cases smartly.
    test!(
        test_overlap,
        ["add", "3-5", "2-4"],
        ["add", "3", "4", "5", "2", "3", "4"]
    );

    // anything after `--` will also be processed. This is for commands
    // like `git reset` which requires pathspecs to appear after --.
    test!(
        test_double_dash,
        ["add", "3-5", "--", "2-4"],
        ["add", "3", "4", "5", "--", "2", "3", "4"]
    );

    test!(test_zeroes_1, ["add", "0"], ["add", "0"]);
    test!(test_zeroes_2, ["add", "0-1"], ["add", "0", "1"]);
    test!(test_zeroes_3, ["add", "0-0"], ["add", "0"]);

    // Filenames containing dashed dates
    test!(test_date_filename, ["add", "2021-01-31"], ["add", "2021-01-31"]);
}
