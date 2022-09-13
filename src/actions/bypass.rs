use std::collections::LinkedList;
use std::process::Command;

pub fn bypass(args: LinkedList<String>) {
    Command::new("git").args(args).spawn().unwrap().wait().unwrap();
}
