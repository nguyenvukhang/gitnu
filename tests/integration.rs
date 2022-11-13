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
fn every_git_state() {
    let mut test = gitnu_test!();
    test.gitnu("", "init");
    for file in "A B C D E F G H I".split(' ') {
        test.shell("", &format!("touch {}", file));
        test.append_to_file(file, file);
    }
    test.gitnu("", "add B C D E G H I")
        .gitnu("", "commit -m pre")
        // modify
        .append_to_file("B", "_")
        .append_to_file("G", "_")
        // remove
        .remove("C")
        // rename
        .rename("E", "_E")
        .rename("I", "_I")
        // typechange
        .shell("", "ln -sf . D")
        .shell("", "ln -sf . H")
        .gitnu("", "add A B C D E _E")
        .gitnu("", "status")
        .expect_stdout(
            "
---
On branch main
Changes to be committed:
1	[32mnew file:   A[m
2	[32mmodified:   B[m
3	[32mdeleted:    C[m
4	[32mtypechange: D[m
5	[32mrenamed:    E -> _E[m

Changes not staged for commit:
6	[31mmodified:   G[m
7	[31mtypechange: H[m
8	[31mdeleted:    I[m

Untracked files:
9	[31mF[m
10	[31m_I[m

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
1  [32mA[m  A
2  [32mA[m  B
3  [31m??[m C
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
fn merge_conflict() {
    let mut test = gitnu_test!();
    test.gitnu("", "init")
        .shell("", "touch base")
        .gitnu("", "add --all")
        .gitnu("", "commit -m base_commit")
        // left branch
        .gitnu("", "branch -m LEFT")
        .shell("", "touch conflict_file")
        .append_to_file("conflict_file", "left")
        .gitnu("", "add --all")
        .gitnu("", "commit -m left_commit")
        // right branch
        .gitnu("", "checkout -b RIGHT")
        .gitnu("", "reset --hard HEAD~1")
        .shell("", "touch conflict_file")
        .write_to_file("conflict_file", "right")
        .gitnu("", "add --all")
        .gitnu("", "commit -m right_commit")
        // merge
        .gitnu("", "merge LEFT")
        .shell("", "touch fileA fileB fileC")
        .gitnu("", "add fileA")
        .gitnu("", "status")
        .gitnu("", "add 3")
        .gitnu("", "status")
        .expect_stdout(
            "
---
On branch RIGHT
You have unmerged paths.

Changes to be committed:
1	[32mnew file:   fileA[m
2	[32mnew file:   fileB[m

Unmerged paths:
3	[31mboth added:      conflict_file[m

Untracked files:
4	[31mfileC[m

",
        );
}
