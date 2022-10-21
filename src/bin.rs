fn main() {
    let (args, opts) = gitnu_lib::core(std::env::args());
    gitnu_lib::run(args, opts);
}
