use gitnu;
use std::path::PathBuf;

fn to_string_vec(s: &[&str]) -> Vec<String> {
    s.iter().map(|v| String::from(*v)).collect()
}

fn to_pathbuf_vec(s: &[&str]) -> Vec<PathBuf> {
    s.iter().map(|v| PathBuf::from(v)).collect()
}

/// test CLI input against gitnu's output args
pub fn test_core(received: &[&str], expected: &[&str]) {
    let (r, _) = gitnu::core(to_string_vec(received));
    let e = to_pathbuf_vec(expected);
    assert_eq!(r, e);
}
