use std::env::{args, current_dir};
use std::process::{Command, ExitCode};

use gitnu::prelude::*;

const DEBUG_MODE: bool = false;

fn main() -> ExitCode {
    let current_dir = current_dir().unwrap_or_default();

    let app = match gitnu::App::new(current_dir) {
        Ok(v) => v,
        Err(_) => {
            return Command::new("git")
                .args(args().skip(1))
                .status()
                .to_err()
                .to_exit_code();
        }
    };

    let app = app.parse(args());
    app.and_then(|mut v| match DEBUG_MODE {
        true => v.debug(),
        false => v.run(),
    })
    .to_exit_code()
}
