mod cache;
mod error;
mod git;
mod git_cmd;
mod parse;
mod pathdiff;
mod prelude;
mod status;
mod traits;

#[cfg(test)]
mod tests;

use prelude::*;

use std::env::{args, current_dir};
use std::path::PathBuf;
use std::process::{Command, ExitCode, ExitStatus};
use std::thread;

fn prefetch(cwd: PathBuf) -> Result<(PathBuf, PathBuf, Aliases)> {
    let h_git_dir = thread::spawn(move || git::dir(&cwd).map(|gd| (gd, cwd)));
    let h_git_aliases = thread::spawn(git::aliases);

    let (git_dir, cwd) = h_git_dir.join()??;
    let git_aliases = h_git_aliases.join()?;
    Ok((cwd, git_dir, git_aliases))
}

pub fn main_cli(cwd: PathBuf, args: &[String]) -> Result<ExitStatus> {
    let (cwd, git_dir, git_aliases) = prefetch(cwd)?;

    let cache = Cache::new(&git_dir, &cwd);

    let mut argh = Command::new("git");
    argh.current_dir(&cwd);
    let (mut argh, git_cmd) = parse::parse(args, git_aliases, cache, argh);

    use GitCommand as G;
    match git_cmd {
        Some(v @ G::Status(_)) => status::git_status(argh, &git_dir, v),
        Some(G::Version) => {
            let result = argh.run();
            println!("gitnu version {CARGO_PKG_VERSION}");
            result
        }
        _ => argh.run(),
    }
}

fn main() -> ExitCode {
    let cwd = current_dir().unwrap_or_default();
    let args = args().collect::<Vec<_>>();
    match main_cli(cwd, &args) {
        Ok(v) => v.to_exitcode(),
        Err(_) => {
            let mut git = Command::new("git");
            git.args(&args[1..]);
            git.status()
                .map_err(Error::from)
                .map(|v| v.to_exitcode())
                .unwrap_or(ExitCode::FAILURE)
        }
    }
}
