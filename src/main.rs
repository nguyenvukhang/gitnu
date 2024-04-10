mod cache;
mod error;
mod git;
mod git_cmd;
mod parse;
mod pathdiff;
mod prelude;
mod status;

#[cfg(test)]
mod tests;

use prelude::*;

use std::env::{args, current_dir};
use std::path::PathBuf;
use std::process::{Command, ExitCode, ExitStatus};
use std::thread;

/// Returning `Err` here means the failure comes from factors outside
/// of `gitnu`. This means we should execute a full bypass to `git` to
/// let it reflect the errors.
fn prefetch(cwd: PathBuf) -> Result<(PathBuf, PathBuf, Aliases)> {
    let h_git_dir = thread::spawn(move || git::dir(&cwd).map(|gd| (gd, cwd)));
    let h_git_aliases = thread::spawn(git::aliases);

    let (git_dir, cwd) = h_git_dir.join()??;
    let git_aliases = h_git_aliases.join()?;
    Ok((cwd, git_dir, git_aliases))
}

/// Return status here does NOT depend on `gitnu` logic. It's purely
/// the result of running the args that `gitnu` parsed.
fn postrun(
    mut cmd: Command,
    git_cmd: Option<GitCommand>,
    git_dir: PathBuf,
) -> Result<ExitStatus> {
    use GitCommand as G;
    match git_cmd {
        // Special case for `git nu status` because that requires
        // __writing__ to the cache.
        Some(v @ G::Status(_)) => status::git_status(cmd, &git_dir, v),
        // For `git version`, append `gitnu`'s version below.
        Some(G::Version) => {
            let result = cmd.run();
            println!("gitnu version {CARGO_PKG_VERSION}");
            result
        }
        // Otherwise, run as parsed.
        _ => cmd.run(),
    }
}

/// A complete run from `cwd` and `args` to the end. Suitable for
/// running `gitnu` entirely during functional tests.
fn main_cli(cwd: PathBuf, args: &[String]) -> Result<ExitStatus> {
    let (cwd, git_dir, git_aliases) = match prefetch(cwd) {
        Ok(v) => v,
        Err(_) => {
            // Run a full bypass
            let mut git = Command::new("git");
            git.args(&args[1..]);
            return git.status().map_err(Error::from);
        }
    };

    let mut argh = Command::new("git");
    argh.current_dir(&cwd);

    let cache = Cache::new(&git_dir, &cwd);
    let (argh, git_cmd) = parse::parse(&args, git_aliases, cache, argh);

    postrun(argh, git_cmd, git_dir)
}

fn main() -> ExitCode {
    let cwd = current_dir().unwrap_or_default();
    let args = args().collect::<Vec<_>>();
    main_cli(cwd, &args).map(|v| v.to_exitcode()).unwrap_or(ExitCode::FAILURE)
}
