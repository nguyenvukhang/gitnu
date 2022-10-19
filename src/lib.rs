mod files;
mod opts;
mod parser;
mod status;

use opts::{Op, Opts};
use std::path::PathBuf;

pub fn core(args: Vec<String>) -> (Vec<PathBuf>, Opts) {
    let (args, opts) = parser::parse(&args);
    let args = files::load(args, &opts);
    (args, opts)
}

pub fn run(args: Vec<PathBuf>, opts: Opts) {
    let result = match opts.op {
        Op::Status => status::run(args, opts),
        _ => opts::run(opts::get_cmd(&opts), args),
    };
    result.ok();
}

#[cfg(test)]
mod tests;
