use std::env::{args, current_dir};
use std::process::ExitCode;

const DEBUG_MODE: bool = false;

fn main() -> ExitCode {
    let cwd = current_dir().unwrap_or_default();
    // TODO: resolve this unwrap
    let app = gitnu::App::new(cwd).unwrap();
    let app = app.parse(args());
    match app.and_then(|mut v| match DEBUG_MODE {
        true => v.debug(),
        false => v.run(),
    }) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => ExitCode::from(e.code()),
    }
}
