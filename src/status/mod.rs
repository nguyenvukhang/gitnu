pub mod backend;
pub mod frontend;
use crate::opts::Opts;
use std::thread;
use std::path::PathBuf;

/// Endpoint function for `gitnu status`
/// Spawns two concurrent threads:
///   1. frontend: displays enumerated status
///   2. backend: populates cache with indexed files for future use
/// These two processes run completely indepedently of each other.
pub fn run(args: Vec<PathBuf>, opts: Opts) {
    let opts1 = opts.clone();
    let frontend = thread::spawn(|| frontend::run(args, opts1));
    let backend = thread::spawn(|| backend::run(opts));
    let wait = |v: thread::JoinHandle<Option<()>>, name| {
        if v.join().is_err() {
            eprintln!("{} thread failed", name);
        }
    };
    // wait for both to finish
    wait(frontend, "frontend");
    wait(backend, "backend");
}
