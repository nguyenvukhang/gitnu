// gitnu status

pub mod backend;
pub mod frontend;
use crate::opts::Opts;
use std::thread;

pub fn run(opts: Opts) {
    let opts1 = opts.clone();
    let frontend = thread::spawn(|| frontend::run(opts1));
    let backend = thread::spawn(|| backend::run(opts));

    // wait for both to finish
    match frontend.join() {
        Ok(_) => (),
        Err(_) => log::info!("frontend thread failed"),
    };
    match backend.join() {
        Ok(_) => (),
        Err(_) => log::info!("backend thread failed"),
    };
}
