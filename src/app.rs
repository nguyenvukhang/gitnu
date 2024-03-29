use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;

use crate::git_cmd::GitCommand;
use crate::prelude::*;
use crate::Cache;

type Aliases = HashMap<String, String>;

#[derive(Debug)]
pub(crate) struct App {
    pub git_aliases: Aliases,
    pub git_cmd: Option<GitCommand>,
    pub git_dir: PathBuf,
    pub cwd: PathBuf,
    pub final_cmd: Command,
    pub cache: Cache,
}

impl Default for App {
    fn default() -> Self {
        Self {
            git_aliases: Aliases::new(),
            git_cmd: None,
            git_dir: PathBuf::new(),
            cwd: PathBuf::new(),
            final_cmd: Command::new("git"),
            cache: Cache::default(),
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
    pub fn parse(&mut self, args: &[String]) -> &mut Self {
        if atty::is(atty::Stream::Stdout) {
            self.final_cmd.args(["-c", "color.ui=always"]);
        }
        let args = &args[1..];
        let args = self.before_cmd(&args);
        self.after_cmd(&args);

        self
    }

    fn before_cmd<'a>(&mut self, mut args: &'a [String]) -> &'a [String] {
        use GitCommand as GC;
        while !args.is_empty() {
            let arg = args[0].as_str();
            args = &args[1..];
            match GC::from_arg(&self.git_aliases, arg) {
                Some(v) => {
                    self.git_cmd = Some(v);
                    self.final_cmd.arg(arg);
                    break;
                }
                _ => {
                    self.final_cmd.arg(arg);
                }
            }
        }
        args
    }

    fn after_cmd(&mut self, args: &[String]) {
        if let None = self.git_cmd {
            self.final_cmd.args(args);
            return;
        }

        for i in 0..args.len() {
            let arg = args[i].as_str();
            let git_cmd = self.git_cmd.as_mut().unwrap();
            match git_cmd {
                GitCommand::Status(ref mut v) => match arg {
                    "--short" | "-s" | "--porcelain" => v.short(),
                    _ => {}
                },
                _ => {}
            };
            let skip = i > 0 && git_cmd.skip_next_arg(&args[i - 1]);
            match (skip, parse_range(&arg)) {
                (false, Some((start, end))) if end <= MAX_CACHE_SIZE => {
                    let cmd = &mut self.final_cmd;
                    (start..end + 1).for_each(|i| self.cache.load(i, cmd));
                }
                _ => {
                    self.final_cmd.arg(&arg);
                }
            }
        }
    }

    pub(crate) fn run(mut self) -> Result<ExitCode> {
        use GitCommand as G;
        let git_cmd = self.git_cmd.clone();
        match git_cmd {
            Some(G::Status(_)) => self.git_status(),
            Some(G::Version) => {
                let result = self.final_cmd.status();
                let exitcode = result.map(|v| v.exitcode())?;
                println!("gitnu version {CARGO_PKG_VERSION}");
                Ok(exitcode)
            }
            _ => {
                let result = self.final_cmd.status();
                let exitcode = result.map(|v| v.exitcode())?;
                Ok(exitcode)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    fn parse(args: &[&str]) -> Vec<String> {
        let mut a = vec!["git"];
        a.extend(args);
        let mut app = App::default();
        app.parse(&string_vec(a));
        app.final_cmd.real_args()
    }

    macro_rules! test {
        ($name:ident, $input_args:expr, $output_args:expr) => {
            #[test]
            fn $name() {
                let received_args = parse(&$input_args);
                let expected_args = string_vec($output_args);
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

    test!(test_zeros_1, ["add", "0"], ["add", "0"]);
    test!(test_zeros_2, ["add", "0-1"], ["add", "0", "1"]);
    test!(test_zeros_3, ["add", "0-0"], ["add", "0"]);

    // Filenames containing dashed dates
    test!(test_date_filename, ["add", "2021-01-31"], ["add", "2021-01-31"]);
}
