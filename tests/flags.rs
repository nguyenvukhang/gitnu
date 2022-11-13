#[test]
fn short_status() {
    let mut test = gitnu_test!();
    test.gitnu("", "init")
        .shell("", "touch A B C")
        .gitnu("", "status -s")
        .gitnu("", "add 1-2")
        .gitnu("", "status -s")
        .expect_stdout(
            "
---
1  A  A
2  A  B
3  ?? C
",
        );
}

#[test]
fn porcelain_status() {
    let mut test = gitnu_test!();
    test.gitnu("", "init")
        .shell("", "touch A B C")
        .gitnu("", "status --porcelain")
        .gitnu("", "add 1-2")
        .gitnu("", "status --porcelain")
        .expect_stdout(
            "
---
1  A  A
2  A  B
3  ?? C
",
        );
}

#[test]
fn skip_short_flags() {
    let mut test = gitnu_test!();
    test.gitnu("", "init")
        .shell("", "touch A B C D E F G H I J")
        .gitnu("", "status");
    for f in "A B C D E F G H I J".split(' ') {
        test.gitnu("", &format!("add {}", f));
        test.gitnu("", &format!("commit -m commit::{}", f));
    }
    // don't parse the 5 after the -n flag because it's likely used as
    // a value to that flag
    //
    // note that 6-8 is still parsed because the flag before them is a
    // long flag
    test.gitnu("", "log -n 5 --pretty=%s 6-8").expect_stdout(
        "
---
commit::H
commit::G
commit::F
",
    );
}

#[test]
fn handle_capital_c_flag() {
    let mut test = gitnu_test!();
    test.shell("", "mkdir one two")
        .shell("", "touch one/one_file two/two_file")
        // populate both repositories' cache
        .gitnu("one", "init")
        .gitnu("one", "status")
        .gitnu("two", "init")
        .gitnu("two", "status")
        // run commands from /two
        .gitnu("two", "-C ../one add 1")
        .gitnu("two", "-C ../one status --porcelain")
        .expect_stdout("1  A  one_file\n")
        .assert()
        .gitnu("two", "add 1")
        .gitnu("two", "status --porcelain")
        .expect_stdout("1  A  two_file\n");
}
