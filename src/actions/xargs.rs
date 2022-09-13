use crate::structs::Gitnu;
use std::collections::LinkedList;
use std::process::Command;

pub fn xargs(
    mut _args: LinkedList<String>,
    _gitnu: &mut Gitnu,
) -> Command {
    let cmd = Command::new("ls");
    cmd
}
