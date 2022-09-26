use gitnu;
use log::info;

fn main() {
    // start logger
    env_logger::builder().format_timestamp(None).init();

    // CLI arguments received
    let args: Vec<std::ffi::OsString> =
        std::env::args().map(|v| v.into()).collect();

    // parse arguments
    let (args, opts) = gitnu::core(args);
    info!("core() -> {:?}", args);

    // run
    gitnu::run(args, opts);
}
