use std::env::{args, current_dir};

fn main() {
    let opts = gitnu::parse(args(), current_dir().unwrap_or_default());
    gitnu::run(opts);
}
