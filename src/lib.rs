mod actions;
mod files;
mod opts;
mod range;
mod status;
use opts::{OpType, Opts};
use std::ffi::OsString;

/// Finally runs the command with all arguments parsed.
/// No further actions after this function is ran.
pub fn run(args: Vec<OsString>, opts: Opts) {
    match opts.op {
        OpType::Status => status::run(opts),
        _ => {
            use actions::RunAction;
            opts.run(args).ok();
        }
    }
}

/// Receives CLI arguments, returns parsed arguments to pass to git
pub fn core(args: Vec<OsString>) -> (Vec<OsString>, Opts) {
    let (opts, args) = opts::get(&args);
    let mut args = range::load(args); // parse ranges
    files::load(&mut args, &opts); // insert filenames
    (args, opts)
}
