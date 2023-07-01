use std::env::{args, current_dir};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cwd = current_dir().unwrap_or_default();
    let app = gitnu::parse(cwd, args());
    match app.and_then(|mut v| v.run()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => ExitCode::from(e.code()),
    }
}
