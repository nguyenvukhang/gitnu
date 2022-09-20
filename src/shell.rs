// get stdout of a command
pub fn get_stdout(
    cmd: &mut std::process::Command,
) -> Result<String, std::io::Error> {
    let trim_out = |p: std::process::Output| {
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
