use std::env::{args, current_dir};
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut app = gitnu::parse(args(), current_dir().unwrap_or_default());
    match app.run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => ExitCode::from(e.code()),
    }
}


// fn main() {
//     println!("memes")
// }
