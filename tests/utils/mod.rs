#![allow(dead_code)]

use gitnu;
use std::ffi::OsString;

pub fn to_osstring_vec(s: &[&str]) -> Vec<OsString> {
    s.iter().map(|v| v.into()).collect()
}

/// test CLI input against gitnu's output args
pub fn test_core(received: &[&str], expected: &[&str]) {
    let (r, _) = gitnu::core(to_osstring_vec(received));
    let e = to_osstring_vec(expected);
    assert_eq!(r, e);
}
