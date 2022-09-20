use gitnu;
use log::info;

fn main() {
    // start logger
    env_logger::builder().format_timestamp(None).init();

    // CLI arguments received
    let args: Vec<String> = std::env::args().collect();

    // parse arguments
    let (args, opts) = gitnu::core(args);
    info!("core() -> {:?}", args);

    // run
    gitnu::run(args, opts);
}
