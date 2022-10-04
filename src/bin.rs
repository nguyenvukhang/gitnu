fn main() {
    let args = std::env::args().collect();
    let (args, opts) = gitnu_lib::core(args);
    gitnu_lib::run(args, opts);
}
