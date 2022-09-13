use gitnu;
use gitnu::structs::OpType;
use std::collections::LinkedList;
use std::env;
use std::process::Command;

fn run(mut cmd: Command) {
    cmd.spawn()
        .expect("unable to spawn gitnu::core's command")
        .wait()
        .expect("unable to wait for gitnu::core's command");
}

fn main() {
    // start logger
    env_logger::builder().format_timestamp(None).init();

    // CLI arguments received
    let args: LinkedList<String> = env::args().collect();

    // start the magic
    let gitnu = gitnu::core(args);

    // quit early if no command to run
    if gitnu.cmd.get_program() == "" {
        return;
    }

    // execute the command
    match gitnu.op {
        OpType::Bypass | OpType::Read | OpType::Xargs => run(gitnu.cmd),
        OpType::Status => gitnu::actions::enumerate(gitnu),
    };
}
