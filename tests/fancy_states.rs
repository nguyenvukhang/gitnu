#[test]
fn every_git_state() {
    let mut test = gitnu_test!();
    test.gitnu("", "init");
    for file in "A B C D E F G H I".split(' ') {
        test.shell("", &format!("touch {}", file));
        test.write_to_file(file, file);
    }
    test.gitnu("", "add B C D E G H I")
        .gitnu("", "commit -m pre")
        // modify
        .write_to_file("B", "B_")
        .write_to_file("G", "G_")
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
fn merge_conflict() {
    let mut test = gitnu_test!();
    test.gitnu("", "init")
        .shell("", "touch base")
        .gitnu("", "add --all")
        .gitnu("", "commit -m base_commit")
        // left branch
        .gitnu("", "branch -m LEFT")
        .shell("", "touch conflict_file")
        .write_to_file("conflict_file", "left")
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
