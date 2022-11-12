use crate::test::test;

#[test]
fn status() {
    test(module_path!(), "status")
        .expect_stderr("")
        .expect_stdout(
            "
---
On branch main

No commits yet

Untracked files:
1	[31mfile1[m
2	[31mfile2[m
3	[31mfile3[m

nothing added to commit but untracked files present
",
        )
        .gitnu("", "init")
        .shell("", "touch file1 file2 file3")
        .gitnu("", "status");
}
