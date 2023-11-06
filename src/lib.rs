mod app;
mod cache;
mod command2;
mod git_cmd;
mod pathdiff;
mod prelude;
mod status;

#[cfg(test)]
mod tests;

use prelude::*;

use std::io::BufRead;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

use app::{App, AppBuilder};
use cache::Cache;
use command2::Command2;

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
    pub(crate) fn relative_dir<P>(base_dir: P) -> Result<PathBuf>
    where
        P: AsRef<Path>,
    {
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

pub fn main<I>(cwd: PathBuf, args: I) -> ExitCode
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
