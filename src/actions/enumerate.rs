use crate::structs::Gitnu;
use std::io::{BufRead, BufReader};
use std::process::Command;

fn has_color(s: &str) -> bool {
    s.contains("\x1b[31m") || s.contains("\x1b[32m")
}

fn print_line(line: &String, count: &mut u16, limit: u16) {
    if has_color(&line) {
        *count += 1;
        if *count > limit {
            return;
        }
        if line.starts_with('\t') {
            println!("{}{}", count, line);
        } else {
            println!("{: <4}{}", count, line);
        }
    } else {
        println!("{}", line);
    }
}

fn print_enumerated_status(cmd: &mut Command) -> Result<(), ()> {
    let mut cmd = cmd.spawn().expect("gitnu::enumerate unable to spawn status");
    let bufin = cmd.stdout.as_mut().expect("gitnu::enumerate has no stdout");

    // only show the first 50 files
    let limit = 50;
    let mut count: u16 = 0;
    BufReader::new(bufin)
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| print_line(&line, &mut count, limit));

    if count > limit {
        println!("... {} hidden items (gitnu)", count - limit);
    }

    if !cmd.wait().unwrap().success() {
        return Err(());
    }

    Ok(())
}

pub fn enumerate(mut gitnu: Gitnu) {
    match print_enumerated_status(&mut gitnu.cmd) {
        Ok(_) => {
            gitnu.load_files();
            gitnu.write_json();
        }
        _ => (),
    };
}
