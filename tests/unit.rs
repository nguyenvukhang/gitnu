mod utils;
use utils::test_core as core;

#[test]
fn status_operations() {
    // removes -C <dir> or -c <binary>
    // and adds in the color
    core(&["gitnu", "status"], &["status"]);
    core(&["gitnu", "-C", "foo/bar", "status"], &["status"]);
    core(&["gitnu", "status", "--porcelain"], &["status", "--porcelain"]);
    core(&["gitnu", "status", "--short"], &["status", "--short"]);
    core(&["gitnu", "-C"], &["-C"]);
    core(&["gitnu", "-c"], &["-c"]);
}

#[test]
fn xargs_operations() {
    core(&["gitnu", "-c", "nvim", "df57bd6565cffb2a"], &["df57bd6565cffb2a"]);
    core(
        &["gitnu", "-c", "nvim", "df57bd65", "65cffb2a"],
        &["df57bd65", "65cffb2a"],
    );
    core(
        &["gitnu", "-c", "cat", "df57bd", "6565cf", "fb2axd"],
        &["df57bd", "6565cf", "fb2axd"],
    );
}
