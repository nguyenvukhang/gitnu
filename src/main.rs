use gitnu; // this points to lib.rs

/// This is what runs when `gitnu` is called from the command line.
///
/// Command line arguments are read using std::env::args(), and then
/// passed to gitnu's functions for processing.
fn main() {
    // gather command line arguments
    let args = std::env::args().collect();

    // parse arguments
    let (args, opts) = gitnu::core(args);

    // run
    gitnu::run(args, opts);
}
