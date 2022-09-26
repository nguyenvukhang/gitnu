mod files;
mod opts;
mod range;
mod shell;
mod status;
use opts::{OpType, Opts};
use std::ffi::OsString;

pub fn run(args: Vec<OsString>, opts: Opts) {
    match opts.op {
        OpType::Status => {
            status::run(opts);
        }
        _ => match opts.run(args) {
            _ => (),
        },
    }
}

// receives CLI arguments, returns parsed arguments to pass to git
pub fn core(args: Vec<OsString>) -> (Vec<OsString>, Opts) {
    let (opts, args) = opts::get(&args);
    let mut args = range::load(args); // parse ranges
    files::load(&mut args, &opts); // insert filenames

    log::info!("parsed args {:?}", args);
    (args, opts)
}
