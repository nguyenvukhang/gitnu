use std::env::{args, current_dir};

fn main() {
    gitnu::parse(args(), current_dir().unwrap_or_default()).run();
}
