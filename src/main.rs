/// This is what runs when `gitnu` is called from the command line.
///
/// Command line arguments are read using std::env::args(), and then
/// passed to gitnu's functions for processing.
fn main() {
    let args = std::env::args().collect();
    let (args, opts) = core(args);
    run(args, opts);
}

mod files;
mod opts;
mod range;
mod status;

use opts::{OpType, Opts};
use std::path::PathBuf;

/// Takes in CLI arguments in a String vector,
/// returns fully parsed arguments to pass to git
pub fn core(args: Vec<String>) -> (Vec<PathBuf>, Opts) {
    use opts::Parser;
    let (args, opts) = Opts::parse(&args);
    let args = range::load(args); // parse ranges
    let args = files::load(args, &opts); // insert filenames
    (args, opts)
}

/// Endpoint function.
/// Runs user-facing command with all arguments parsed.
/// No further actions after this function is ran.
pub fn run(args: Vec<PathBuf>, opts: Opts) {
    match opts.op {
        OpType::Status => {
            status::run(args, opts);
        }
        _ => {
            use opts::Commands;
            opts.run(args).ok();
        }
    }
}

/// test CLI input against gitnu's output args
pub fn test(received: &[&str], expected: &[&str]) {
    fn to_string_vec(s: &[&str]) -> Vec<String> {
        s.iter().map(|v| String::from(*v)).collect()
    }
    fn to_pathbuf_vec(s: &[&str]) -> Vec<PathBuf> {
        s.iter().map(|v| PathBuf::from(v)).collect()
    }
    let (r, _) = core(to_string_vec(received));
    let e = to_pathbuf_vec(expected);
    assert_eq!(r, e);
}

#[test]
fn status_operations() {
    // removes -C <dir> or -c <binary>
    // and adds in the color
    test(&["gitnu", "status"], &["status"]);
    test(&["gitnu", "-C", "foo/bar", "status"], &["status"]);
    test(&["gitnu", "status", "--porcelain"], &["status", "--porcelain"]);
    test(&["gitnu", "status", "--short"], &["status", "--short"]);
    test(&["gitnu", "-C"], &["-C"]);
    test(&["gitnu", "-c"], &["-c"]);
}

#[test]
fn xargs_operations() {
    test(&["gitnu", "-c", "nvim", "df57bd6565cffb2a"], &["df57bd6565cffb2a"]);
    test(
        &["gitnu", "-c", "nvim", "df57bd65", "65cffb2a"],
        &["df57bd65", "65cffb2a"],
    );
    test(
        &["gitnu", "-c", "cat", "df57bd", "6565cf", "fb2axd"],
        &["df57bd", "6565cf", "fb2axd"],
    );
}
