// gitnu status

pub mod backend;
pub mod frontend;
use crate::opts::Opts;
use std::thread;

pub fn run(opts: Opts) {
    let opts1 = opts.clone();
    let frontend = thread::spawn(|| frontend::run(opts1));
    let backend = thread::spawn(|| backend::run(opts));
    let wait = |v: thread::JoinHandle<Result<(), std::io::Error>>, name| {
        if v.join().is_err() {
            eprintln!("{} thread failed", name);
        }
    };
    // wait for both to finish
    wait(frontend, "frontend");
    wait(backend, "backend");
}
