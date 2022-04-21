use std::process::Command;
use std::io::{self, Write};

fn main() {
    println!("Gonna git good");
    let mut git = Command::new("git");
    let status = git.arg("status");
    let hello = status.output().expect("failed to execute command.");
    println!("Command output: {}", hello.status);
    io::stdout().write_all(&hello.stdout).unwrap();
    io::stderr().write_all(&hello.stderr).unwrap();
}
