use std::env::{args, current_dir};
use std::process::ExitCode;

fn main() -> ExitCode {
    let current_dir = current_dir().unwrap_or_default();
    gitnu::main(current_dir, &args().collect::<Vec<_>>())
}
