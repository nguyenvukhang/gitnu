mod git;
mod app;
mod error;
mod cache;
mod git_cmd;
mod pathdiff;
mod prelude;
mod status;

#[cfg(test)]
mod tests;

use prelude::*;

use std::env::{args, current_dir};
use std::io::BufRead;
use std::path::PathBuf;
use std::process::{Command, ExitCode};

use app::App;
use cache::Cache;


fn cli_init_app(cwd: PathBuf) -> Result<App> {
    use std::thread;

    let h_git_dir = thread::spawn(move || git::dir(&cwd).map(|gd| (gd, cwd)));
    let h_git_aliases = thread::spawn(git::aliases);

    let (git_dir, cwd) = h_git_dir.join()??;
    let git_aliases = h_git_aliases.join()?;

    let cache = Cache::new(&git_dir, &cwd);

    let mut final_cmd = Command::new("git");
    final_cmd.current_dir(&cwd);

    Ok(App { git_aliases, git_cmd: None, git_dir, cwd, final_cmd, cache })
}

pub fn main_inner(cwd: PathBuf, args: &[String]) -> ExitCode {
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

fn main() -> ExitCode {
    let current_dir = current_dir().unwrap_or_default();
    main_inner(current_dir, &args().collect::<Vec<_>>())
}
