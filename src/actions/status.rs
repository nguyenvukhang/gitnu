use std::collections::LinkedList;
use std::process::{Command, Stdio};

fn prep_enum_status(
    rargs: LinkedList<String>,
    mut largs: LinkedList<String>,
) -> Command {

    // add color
    let cmd = largs.pop_back().unwrap();
    largs.push_back("-c".to_string());
    largs.push_back("status.color=always".to_string());
    largs.push_back(cmd);

    // push args into largs
    largs.extend(rargs);

    // create a child process loaded with the args
    let mut cmd = Command::new("git");
    cmd.args(largs);
    return cmd;
}

fn tmp_unsupported_flags(rargs: &LinkedList<String>) -> bool {
    fn msg(flag: &str) -> bool {
        println!("\x1b[37m`gitnu status {}` is not supported yet\x1b[0m", flag);
        false
    }
    for i in rargs.iter() {
        match i.as_str() {
            "--short" | "-s" => return msg("--short | -s"),
            "--porcelain" => return msg("--porcelain"),
            _ => (),
        };
    }
    true
}

pub fn status(
    rargs: LinkedList<String>,
    mut largs: LinkedList<String>,
) -> Command {
    let mut cmd = Command::new("git");
    if !tmp_unsupported_flags(&rargs) {
        // bypass
        largs.extend(rargs);
        cmd.args(largs);
    } else {
        cmd = prep_enum_status(rargs, largs);
    }
    cmd.stdout(Stdio::piped());
    cmd
}
