pub mod actions;
pub mod shell;
pub mod structs;
use crate::structs::{Gitnu, OpType};
use log::info;
use std::collections::LinkedList;
use std::process::Command;

// gitnu will only trigger on these commands
fn is_supported_git_command(arg: &str) -> Option<OpType> {
    match arg {
        "add" | "reset" | "diff" | "checkout" => Some(OpType::Read),
        "status" => Some(OpType::Status),
        _ => None,
    }
}

// args: the original CLI args passed into gitnu
// largs: processed args
//
// this function works on the (valid) assumption that
// all positional arguments to the left of the command
// should not be processed by gitnu
pub fn split_args_by_cmd(
    largs: &mut LinkedList<String>,
    rargs: &mut LinkedList<String>,
) -> (OpType, String, String) {
    let mut git_dir = String::from('.');
    let mut xargs_cmd = String::from('-');
    let op = OpType::Bypass;
    while !rargs.is_empty() {
        let arg = rargs.pop_front().unwrap();
        // terminate immmediately when a git command is found
        if let Some(op) = is_supported_git_command(&arg) {
            largs.push_back(arg);
            return (op, git_dir, xargs_cmd);
        }
        // the checks below require the existence of a next arg
        if rargs.front().is_none() {
            largs.push_back(arg);
            continue;
        }
        // if arg is -C or -c, remove both that and the next
        // arg from the argument list
        if arg.eq("-C") {
            git_dir = rargs.pop_front().unwrap();
            continue;
        }
        if arg.eq("-c") {
            xargs_cmd = rargs.pop_front().unwrap();
            continue;
        }
        // send the arg to the processed list
        largs.push_back(arg);
    }

    match xargs_cmd.eq("-") {
        true => (op, git_dir, xargs_cmd),
        false => (OpType::Xargs, git_dir, xargs_cmd),
    }
}

#[test]
fn test_split_args_by_cmd() {
    fn to_ll(l: &[&str]) -> LinkedList<String> {
        l.to_vec().into_iter().map(|e| String::from(e)).collect()
    }
    fn get_outputs(l: &[&str]) -> (OpType, String, String) {
        let mut v = to_ll(l);
        split_args_by_cmd(&mut LinkedList::new(), &mut v)
    }
    fn enum_eq(a: &OpType, b: &OpType) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }
    fn test(l: &[&str], e_op: OpType, e_cwd: &str, e_xargs_cmd: &str) {
        let (op, cwd, xargs_cmd) = get_outputs(l);
        assert_eq!(enum_eq(&op, &e_op), true);
        assert_eq!(cwd, e_cwd);
        assert_eq!(xargs_cmd, e_xargs_cmd);
    }
    test(&["status"], OpType::Status, ".", "-");
    test(&["-C", "/tmp", "status"], OpType::Status, "/tmp", "-");
    test(&["-C", "/bin", "add", "1"], OpType::Read, "/bin", "-");
    test(&["-c", "cat", "1"], OpType::Xargs, ".", "cat");
    test(&["-C", "/tmp", "-c", "cat", "1"], OpType::Xargs, "/tmp", "cat");
    test(&["-c", "cat", "-C", "/tmp", "1"], OpType::Xargs, "/tmp", "cat");
}

/// takes in a list of CLI arguments
/// lists starts with the name of this binary
pub fn core(mut rargs: LinkedList<String>) -> Gitnu {
    // pop off the name of binary
    rargs.pop_front();

    // enumerated CLI arguments to pass to git
    let mut largs: LinkedList<String> = LinkedList::new();
    let (op, git_dir, xargs_cmd) = split_args_by_cmd(&mut largs, &mut rargs);

    // the one and only instantiation of Gitnu
    let mut gitnu = Gitnu::new(op, git_dir);

    // straight bypass to git with all arguments passed
    let bypass = |args: LinkedList<String>| {
        let mut git = Command::new("git");
        git.args(args);
        return git;
    };

    gitnu.cmd = match gitnu.op {
        OpType::Bypass => {
            info!("lib::bypass");
            bypass(largs)
        }
        OpType::Read => match gitnu.read_json() {
            Ok(_) => {
                info!("lib::read + json");
                gitnu.cmd = Command::new("git");
                actions::read(rargs, largs, &mut gitnu)
            }
            Err(_) => {
                info!("lib::read + no json");
                largs.append(&mut rargs);
                bypass(largs)
            }
        },
        OpType::Status => {
            info!("lib::status");
            actions::status(rargs, largs)
        }
        OpType::Xargs => match gitnu.read_json() {
            Ok(_) => {
                info!("lib::xargs + json");
                // send rargs to largs to empty out rargs
                largs.append(&mut rargs);
                gitnu.cmd = Command::new(xargs_cmd);
                // largs contains all args, rargs is empty
                let res = actions::read(largs, rargs, &mut gitnu);
                res
            }
            Err(_) => {
                info!("lib::xargs + no json");
                largs.append(&mut rargs);
                bypass(largs)
            }
        },
    };
    gitnu
}
