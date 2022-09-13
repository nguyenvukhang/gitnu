mod utils;
use utils::test;

#[test]
fn status_operations() {
    // removes -C <dir> or -c <binary>
    // and adds in the color
    test(vec!["gitnu", "status"], vec!["-c", "status.color=always", "status"]);
    test(
        vec!["gitnu", "-C", "foo/bar", "status"],
        vec!["-c", "status.color=always", "status"],
    );
    test(vec!["gitnu", "status", "--porcelain"], vec!["status", "--porcelain"]);
    test(vec!["gitnu", "status", "--short"], vec!["status", "--short"]);
    test(vec!["gitnu", "-C"], vec!["-C"]);
    test(vec!["gitnu", "-c"], vec!["-c"]);
}

#[test]
fn xargs_operations() {
    test(
        vec!["gitnu", "-c", "nvim", "df57bd6565cffb2a"],
        vec!["df57bd6565cffb2a"],
    );
    test(
        vec!["gitnu", "-c", "nvim", "df57bd65", "65cffb2a"],
        vec!["df57bd65", "65cffb2a"],
    );
    test(
        vec!["gitnu", "-c", "cat", "df57bd", "6565cf", "fb2axd"],
        vec!["df57bd", "6565cf", "fb2axd"],
    );
}
