use std::env::{args, current_dir};
use std::process::ExitCode;

fn main() -> ExitCode {
    let current_dir = current_dir().unwrap_or_default();
    let args = args();
    gitnu::main(current_dir, args)
}
