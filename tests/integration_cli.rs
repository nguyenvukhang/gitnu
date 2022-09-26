mod utils;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use utils::{to_osstring_vec, Git};

fn basenames(args: Vec<OsString>) -> Vec<OsString> {
    args.iter()
        .map(|v| {
            PathBuf::from(v).file_name().unwrap_or(OsStr::new("")).to_owned()
        })
        .collect()
}

#[cfg(test)]
pub fn test(dir: &str, received: &[&str], expected: &[&str]) {
    let mut args: Vec<OsString> = to_osstring_vec(&["-C", dir]);
    args.append(&mut to_osstring_vec(received));
    let (r, _) = gitnu::core(args);
    let e = to_osstring_vec(expected);
    // convert both to PathBuf vectors
    let r = basenames(r);
    let e = basenames(e);
    println!("R = {:?}", r);
    assert_eq!(r, e);
}

#[cfg(test)]
fn setup(dir: &str, files: u16) {
    let (mut git, mut gitnu) = Git::gpair(dir);
    git.init();
    for i in 1..files + 1 {
        git.touch(&format!("file{}", i));
    }
    gitnu.status();
}

#[test]
fn enumerated_cli() {
    let d = "tmp/c1";
    setup(d, 1);
    test(d, &["gitnu", "add", "1"], &["add", "file1"]);

    let d = "tmp/c2";
    setup(d, 7);
    test(d, &["gitnu", "add", "3-5"], &["add", "file3", "file4", "file5"]);

    let d = "tmp/c3";
    setup(d, 7);
    test(
        d,
        &["gitnu", "add", "2", "4-6"],
        &["add", "file2", "file4", "file5", "file6"],
    );
}
