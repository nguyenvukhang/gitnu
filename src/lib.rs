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

use app::App;
use cache::Cache;
use command2::Command2;

mod git {
    use std::path::Path;
    use std::process::Output;

    use super::*;

    /// Path to git's repository (not workspace)
    ///   * .git/
    ///   * .git/worktrees/<branch-name>/
    ///
    /// current_dir is intentionally not supplied as it relies on the
    /// user's actual PWD or the value of git's `-C` flag, which is not
    /// visible to the `git-nu` binary.
    pub(crate) fn dir<P>(cwd: P) -> Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        _dir(Some(cwd))
    }

    fn sh<P>(dir: Option<P>, args: &[&str]) -> Result<Output>
    where
        P: AsRef<Path>,
    {
        let mut cmd = Command::new("git");
        dir.map(|v| cmd.current_dir(v));
        Ok(cmd.args(args).output().map_err(|e| Error::Io(e))?)
    }

    fn _dir<P: AsRef<Path>>(base_dir: Option<P>) -> Result<PathBuf> {
        let output = git::sh(base_dir, &["rev-parse", "--git-dir"])?;
        if output.stderr.starts_with(b"fatal: not a git repository") {
            return error!(NotGitRepository);
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(PathBuf::from(stdout.trim_end()))
    }

    pub(crate) fn aliases() -> Aliases {
        let args = ["config", "--global", "--get-regexp", "^alias."];
        match git::sh(None::<&str>, &args) {
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

    let h_git_dir = thread::spawn(move || git::dir(&cwd).map(|gd| (gd, cwd)));
    let h_git_aliases = thread::spawn(|| git::aliases());

    let (git_dir, cwd) = h_git_dir.join()??;
    let git_aliases = h_git_aliases.join()?;

    let cache = Cache::new(&git_dir, &cwd);

    Ok(App {
        git_aliases,
        git_cmd: None,
        git_dir,
        cwd,
        final_command: Command2::new("git"),
        cache,
    })
}

pub fn main(cwd: PathBuf, args: &[String]) -> ExitCode {
    let exitcode = match cli_init_app(cwd) {
        Ok(mut app) => {
            app.parse(&args);
            app.run()
        }
        Err(_) => Command::new("git")
            .args(&args[1..])
            .status()
            .map_err(|v| Error::from(v))
            .map(|v| v.exitcode()),
    };
    exitcode.unwrap_or(ExitCode::FAILURE)
}
