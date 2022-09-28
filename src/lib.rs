mod files;
mod opts;
mod range;
mod status;

use opts::{OpType, Opts};
use std::path::PathBuf;

/// Finally runs the command with all arguments parsed.
/// No further actions after this function is ran.
pub fn run(args: Vec<PathBuf>, opts: Opts) {
    match opts.op {
        OpType::Status => status::run(args, opts),
        _ => {
            use opts::RunAction;
            opts.run(args).ok();
        }
    }
}

/// Receives CLI arguments, returns parsed arguments to pass to git
pub fn core(args: Vec<String>) -> (Vec<PathBuf>, Opts) {
    use opts::Parser;
    let (args, opts) = Opts::parse(&args);
    let args = range::load(args); // parse ranges
    let args = files::load(args, &opts); // insert filenames
    (args, opts)
}
