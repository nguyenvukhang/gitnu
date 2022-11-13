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
1	[31mfile-3[m
2	[31mfile:1[m
3	[31mfile_2[m

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
1	[32mnew file:   file_2[m

Untracked files:
2	[31mfile-3[m
3	[31mfile:1[m

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
1	[32mnew file:   B[m
2	[32mnew file:   C[m
3	[32mnew file:   D[m
4	[32mnew file:   F[m

Untracked files:
5	[31mA[m
6	[31mE[m
7	[31mG[m

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
1	[32mnew file:   A[m
2	[32mnew file:   B[m
3	[32mnew file:   C[m
4	[32mnew file:   D[m
5	[32mnew file:   E[m
6	[32mnew file:   F[m

Untracked files:
7	[31mG[m

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
1	[31mA[m
2	[31mB[m
3	[31mC[m

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
1	[32mnew file:   A[m

Untracked files:
2	[31mB[m
3	[31mC[m

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
1	[32mnew file:   ../file3[m
2	[32mnew file:   emerald[m
3	[32mnew file:   ruby[m

Untracked files:
4	[31m../file1[m
5	[31m../file2[m
6	[31msapphire[m

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
1	[32mnew file:   file3[m
2	[32mnew file:   src/emerald[m
3	[32mnew file:   src/ruby[m

Untracked files:
4	[31mfile1[m
5	[31mfile2[m
6	[31msrc/sapphire[m

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
