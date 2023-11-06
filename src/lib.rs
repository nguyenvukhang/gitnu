mod app;
mod git_cmd;
mod pathdiff;
mod prelude;
mod status;

#[cfg(test)]
mod tests;

use prelude::*;

use std::ffi::OsStr;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;

use app::{App, AppBuilder};

#[derive(Debug, Default)]
struct Cache {
    prefix: Option<PathBuf>,
    files: Vec<String>,
}

impl Cache {
    pub fn new(git_dir: &PathBuf, cwd: &PathBuf) -> Self {
        Self::try_read(git_dir, cwd).unwrap_or_default()
    }

    fn try_read(git_dir: &PathBuf, cwd: &PathBuf) -> Result<Self> {
        let mut filepath = cwd.clone();
        filepath.push(git_dir);
        filepath.push(CACHE_FILE_NAME);

        let file = File::open(filepath)?;
        let mut lines = BufReader::new(file).lines().filter_map(|v| v.ok());

        let prefix = {
            let first_line = lines.next().ok_or(Error::InvalidCache)?;
            let prefix = PathBuf::from(first_line);
            match pathdiff::diff_paths(prefix, cwd) {
                Some(v) if v.as_os_str().is_empty() => None,
                v => v,
            }
        };

        let mut files = Vec::with_capacity(MAX_CACHE_SIZE);
        files.push(0.to_string());
        files.extend(lines.take(MAX_CACHE_SIZE - 1));

        Ok(Self { prefix, files })
    }

    pub fn load(&self, index: usize, cmd: &mut Command) {
        if let Some(pathspec) = self.files.get(index) {
            if let Some(prefix) = &self.prefix {
                cmd.arg(prefix.join(pathspec));
            } else {
                cmd.arg(pathspec);
            }
        } else {
            cmd.arg(index.to_string());
        }
    }
}

mod git {
    use std::path::Path;

    use super::*;

    /// Path to git's repository (not workspace)
    ///   * .git/
    ///   * .git/worktrees/<branch-name>/
    ///
    /// current_dir is intentionally not supplied as it relies on the
    /// user's actual PWD or the value of git's `-C` flag, which is not
    /// visible to the `git-nu` binary.
    pub(crate) fn dir() -> Result<PathBuf> {
        _dir(None::<&Path>)
    }

    #[cfg(test)]
    pub(crate) fn relative_dir(base_dir: &PathBuf) -> Result<PathBuf> {
        _dir(Some(base_dir))
    }

    fn _dir<P: AsRef<Path>>(base_dir: Option<P>) -> Result<PathBuf> {
        let mut cmd = Command::new("git");
        if let Some(current_dir) = base_dir {
            cmd.current_dir(current_dir);
        }
        let output = cmd
            .args(["rev-parse", "--git-dir"])
            .output()
            .map_err(|e| Error::Io(e))?;
        if output.stderr.starts_with(b"fatal: not a git repository") {
            return error!(NotGitRepository);
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(PathBuf::from(stdout.trim_end()))
    }

    pub(crate) fn aliases() -> Aliases {
        let output = Command::new("git")
            .args(["config", "--global", "--get-regexp", "^alias."])
            .output();
        match output {
            Ok(v) => Aliases::from_iter(
                v.stdout.lines().filter_map(|v| v.ok()).filter_map(|v| {
                    v.get(6..) // every lines starts with "alias."
                        .and_then(|v| v.split_once(' '))
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                }),
            ),
            Err(_) => Aliases::new(),
        }
    }
}

#[derive(Debug)]
struct Command2 {
    pub inner: Command,
    pub hidden_args: Vec<usize>,
}

impl Command2 {
    pub fn new(program: &str) -> Self {
        Self {
            inner: Command::new(program),
            hidden_args: Vec::with_capacity(2),
        }
    }

    pub fn hidden_args<'a, I>(&mut self, args: I)
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut n = self.inner.get_args().len();
        for arg in args {
            self.inner.arg(arg);
            self.hidden_args.push(n);
            n += 1;
        }
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) {
        self.inner.arg(arg);
    }

    #[cfg(test)]
    pub fn get_args(&self) -> Vec<&str> {
        let mut hidden = self.hidden_args.clone();
        hidden.reverse();

        let mut i = 0usize;
        let mut args = vec![];

        for arg in self.inner.get_args() {
            if hidden.last() == Some(&i) {
                hidden.pop();
            } else if let Some(v) = arg.to_str() {
                args.push(v);
            }
            i += 1;
        }

        args
    }
}

fn cli_init_app(cwd: PathBuf) -> Result<App> {
    use std::thread;

    let h_git_dir = thread::spawn(|| git::dir());
    let h_git_aliases = thread::spawn(|| git::aliases());

    let git_dir = h_git_dir.join()??;
    let git_aliases = h_git_aliases.join()?;

    let cache = Cache::new(&git_dir, &cwd);

    Ok(AppBuilder::new(cwd)
        .git_dir(git_dir)
        .git_aliases(git_aliases)
        .cache(cache)
        .build())
}

pub fn run<I>(cwd: PathBuf, args: I) -> ExitCode
where
    I: IntoIterator<Item = String>,
{
    let args = args.into_iter();

    let exitcode = match cli_init_app(cwd) {
        Ok(app) => app.parse(args).run(),
        Err(_) => Command::new("git")
            .args(args.skip(1))
            .status()
            .map_err(|v| Error::from(v))
            .map(|v| v.exitcode()),
    };
    exitcode.unwrap_or(ExitCode::FAILURE)
}
