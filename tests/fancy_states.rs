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
1	new file:   A
2	modified:   B
3	deleted:    C
4	typechange: D
5	renamed:    E -> _E

Changes not staged for commit:
6	modified:   G
7	typechange: H
8	deleted:    I

Untracked files:
9	F
10	_I

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
1	new file:   fileA
2	new file:   fileB

Unmerged paths:
3	both added:      conflict_file

Untracked files:
4	fileC

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
HEAD detached at [:SHA:]
nothing to commit, working tree clean
",
        );
}
