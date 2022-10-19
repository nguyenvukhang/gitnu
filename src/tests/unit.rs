#![cfg(test)]
use crate::core;
use std::path::PathBuf;

/// test CLI input against gitnu's output args
#[cfg(test)]
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
    test(&["gitnu", "status"], &["status"]);
    test(&["gitnu", "-C", "foo/bar", "status"], &["status"]);
    test(&["gitnu", "status", "--porcelain"], &["status", "--porcelain"]);
    test(&["gitnu", "status", "--short"], &["status", "--short"]);
    test(&["gitnu", "-C"], &["-C"]);
    test(&["gitnu", "-c"], &["-c"]);
    test(&["gitnu", "log", "-n", "2"], &["log", "-n", "2"]);
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