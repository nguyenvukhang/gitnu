use std::{env, path::PathBuf};

fn main() {
    let opts = gitnu_lib::core(
        env::args(),
        env::current_dir().unwrap_or(PathBuf::from(".")),
    );
    gitnu_lib::run(opts);
}
