mod files;
mod opts;
mod range;
mod shell;
mod status;
use opts::{OpType, Opts};

fn spawn(args: Vec<String>, opts: Opts) {
    // get cmd
    let mut cmd = match opts.cmd() {
        Ok(mut cmd) => {
            cmd.args(args);
            cmd
        }
        Err(_) => return,
    };

    // spawn cmd
    let mut cmd = match cmd.spawn() {
        Ok(v) => v,
        Err(_) => return,
    };

    // wait for end
    match cmd.wait() {
        Ok(_) => (),
        Err(_) => (),
    }
}

pub fn run(args: Vec<String>, opts: Opts) {
    match opts.op {
        OpType::Status => {
            status::run(opts);
        }
        OpType::Xargs | OpType::Read | OpType::Bypass => {
            spawn(args, opts);
        }
    }
}

// receives CLI arguments, returns parsed arguments to pass to git
pub fn core(args: Vec<String>) -> (Vec<String>, Opts) {
    let (opts, args) = opts::get(&args);
    let mut args = range::load(args); // parse ranges
    files::load(&mut args, &opts); // insert filenames

    log::info!("parsed args {:?}", args);
    (args, opts)
}
