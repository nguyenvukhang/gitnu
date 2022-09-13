use std::io::Error;
use std::process::{Command, Output};

pub fn get_stdout(cmd: &mut Command) -> Result<String, Error> {
    let trim_out = |p: Output| {
        let mut s = String::from(String::from_utf8_lossy(&p.stdout));
        while s.ends_with('\n') || s.ends_with('\r') {
            s.pop();
        }
        Ok(s)
    };
    match cmd.output() {
        Ok(p) => return trim_out(p),
        Err(e) => return Err(e),
    };
}
