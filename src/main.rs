use gitnu;

fn main() {
    // CLI arguments received
    let args: Vec<std::ffi::OsString> =
        std::env::args().map(|v| v.into()).collect();

    // parse arguments
    let (args, opts) = gitnu::core(args);

    // run
    gitnu::run(args, opts);
}
