use gitnu;

fn main() {
    // CLI arguments received
    let args = std::env::args().collect();

    // parse arguments
    let (args, opts) = gitnu::core(args);

    // run
    gitnu::run(args, opts);
}
