mod utils;
use utils::test_core as test;

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
