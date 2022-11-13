#[test]
fn untracked_files() {
    gitnu_test!()
        .gitnu("", "init")
        .shell("", "touch file:1 file_2 file-3")
        .gitnu("", "status")
        .expect_stdout(
            "
---
On branch main

No commits yet

Untracked files:
1	file-3
2	file:1
3	file_2

nothing added to commit but untracked files present
",
        );
}

#[test]
fn staging_files_with_filename() {
    gitnu_test!()
        .gitnu("", "init")
        .shell("", "touch file:1 file_2 file-3")
        .gitnu("", "add file_2") // use filename
        .gitnu("", "status")
        .expect_stdout(
            "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   file_2

Untracked files:
2	file-3
3	file:1

",
        );
}

#[test]
fn staging_files_with_numbers() {
    gitnu_test!()
        .gitnu("", "init")
        .shell("", "touch A B C D E F G")
        .gitnu("", "status")
        .gitnu("", "add 2-4 6") // use number and range
        .gitnu("", "status")
        .expect_stdout(
            "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   B
2	new file:   C
3	new file:   D
4	new file:   F

Untracked files:
5	A
6	E
7	G

",
        );
}

#[test]
fn range_overlap() {
    let mut test = gitnu_test!();
    test.gitnu("", "init")
        .shell("", "touch A B C D E F G")
        .gitnu("", "status")
        .gitnu("", "add 1-4 2-6")
        .gitnu("", "status")
        .expect_stdout(
            "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   A
2	new file:   B
3	new file:   C
4	new file:   D
5	new file:   E
6	new file:   F

Untracked files:
7	G

",
        );
}

/// Just as git would respond, no files will be added at all
#[test]
fn add_unindexed_number() {
    let mut test = gitnu_test!();
    test.gitnu("", "init")
        .shell("", "touch A B C")
        .gitnu("", "status")
        .gitnu("", "add 2-5") // 2-5 out of range
        .gitnu("", "status")
        .expect_stdout(
            "
---
On branch main

No commits yet

Untracked files:
1	A
2	B
3	C

nothing added to commit but untracked files present
",
        );
}

/// Just as git would respond, some files will be reset
#[test]
fn reset_unindexed_number() {
    let mut test = gitnu_test!();
    test.gitnu("", "init")
        .shell("", "touch A B C")
        .gitnu("", "add --all")
        .gitnu("", "status")
        .gitnu("", "reset 2-5") // 2-5 out of range
        .gitnu("", "status")
        .expect_stdout(
            "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   A

Untracked files:
2	B
3	C

",
        );
}

#[test]
fn not_from_git_root() {
    let mut test = gitnu_test!();
    test.gitnu("", "init")
        .shell("", "mkdir src")
        .shell("", "touch file1 file2 file3 src/.gitkeep")
        .gitnu("", "add src")
        .gitnu("", "commit -m src_dir")
        .shell("src", "touch emerald sapphire ruby")
        .gitnu("src", "status")
        .gitnu("src", "add 3-5")
        .gitnu("src", "status")
        .expect_stdout(
            "
---
On branch main
Changes to be committed:
1	new file:   ../file3
2	new file:   emerald
3	new file:   ruby

Untracked files:
4	../file1
5	../file2
6	sapphire

",
        );
}
#[test]
fn change_cwd_after_status() {
    let mut test = gitnu_test!();
    test.gitnu("", "init")
        .shell("", "mkdir src")
        .shell("", "touch file1 file2 file3 src/.gitkeep")
        .gitnu("", "add src")
        .gitnu("", "commit -m src_dir")
        .shell("src", "touch emerald sapphire ruby")
        .gitnu("src", "status") // run status command in /src
        .gitnu("", "add 3-5") // run add command from /
        .gitnu("", "status")
        .expect_stdout(
            "
---
On branch main
Changes to be committed:
1	new file:   file3
2	new file:   src/emerald
3	new file:   src/ruby

Untracked files:
4	file1
5	file2
6	src/sapphire

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
fn many_files() {
    use crate::data::LONG_EXPECT;
    let mut test = gitnu_test!();
    test.shell(
        "",
        &(1..1000)
            .map(|v| format!(" {:0width$}", v, width = 5))
            .fold(String::from("touch"), |a, v| a + &v),
    );
    test.gitnu("", "init")
        .gitnu("", "status")
        .gitnu("", "add 69-420")
        .gitnu("", "status")
        .expect_stderr("")
        .expect_stdout(LONG_EXPECT);
}
