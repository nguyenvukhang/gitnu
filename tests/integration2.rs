#[test]
fn detached_head() {
    let mut test = gitnu_test!();
    test.gitnu("", "init")
        .shell("", "touch file1")
        .gitnu("", "add --all")
        .gitnu("", "commit -m add::file1")
        .shell("", "touch file2")
        .gitnu("", "add --all")
        .gitnu("", "commit -m add::file2")
        .gitnu("", "checkout HEAD~1")
        .set_sha()
        .gitnu("", "status")
        .expect_stdout(
            "
---
[31mHEAD detached at [m[:SHA:]
nothing to commit, working tree clean
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
fn zero_handling() {
    let mut test = gitnu_test!();
    test.gitnu("", "init")
        .expect_stdout("1  ?? A\n2  ?? B\n")
        .shell("", "touch A B")
        .gitnu("", "status")
        .gitnu("", "add 0")
        .gitnu("", "status --porcelain")
        .assert()
        .gitnu("", "add 0-1")
        .gitnu("", "status --porcelain")
        .assert()
        .gitnu("", "add 0-0")
        .gitnu("", "status --porcelain")
        .assert();
}

#[test]
fn zero_filename() {
    let mut test = gitnu_test!();
    test.gitnu("", "init")
        .shell("", "touch 0 A B")
        .gitnu("", "status --porcelain")
        .expect_stdout(
            "
---
1  ?? 0
2  ?? A
3  ?? B
",
        )
        .assert()
        .gitnu("", "add 0-2")
        .gitnu("", "status --porcelain")
        .expect_stdout(
            "
---
1  A  0
2  A  A
3  ?? B
",
        )
        .assert();
}

#[test]
fn dont_create_cache_file_without_repo() {
    let mut test = gitnu_test!();
    test.gitnu("", "status").shell("", "ls -lA").expect_stdout("total 0\n");
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
