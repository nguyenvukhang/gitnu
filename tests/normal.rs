macro_rules! test {
    ($name:ident, $fun:expr) => {
        gitnu_test!($name, $fun, "");
    };
}

test!(untracked_files, |mut t: Test| {
    t.gitnu("", "init")
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
});

test!(staging_files_with_filename, |mut t: Test| {
    t.gitnu("", "init")
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
});

test!(staging_files_with_numbers, |mut t: Test| {
    t.gitnu("", "init")
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
});

test!(range_overlap, |mut t: Test| {
    t.gitnu("", "init")
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
});

// Just as git would respond, no files will be added at all
test!(add_unindexed_number, |mut t: Test| {
    t.gitnu("", "init")
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
});

// Just as git would respond, some files will be reset

test!(reset_unindexed_number, |mut t: Test| {
    t.gitnu("", "init")
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
});

test!(not_from_git_root, |mut t: Test| {
    t.gitnu("", "init")
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
});

test!(change_cwd_after_status, |mut t: Test| {
    t.gitnu("", "init")
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
});

test!(zero_handling, |mut t: Test| {
    t.gitnu("", "init")
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
});

test!(zero_filename, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch 0 A B")
        .gitnu("", "status")
        .expect_stdout(
            "
---
On branch main

No commits yet

Untracked files:
1	0
2	A
3	B

nothing added to commit but untracked files present
",
        )
        .assert()
        // expand `gitnu add 0-2` to `git add 0 1 2`
        // and then convert it to filenames: `git add 0 0 A`
        // therefore only adding files `0` and `A`
        .gitnu("", "add 0-2")
        .gitnu("", "status")
        .expect_stdout(
            "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   0
2	new file:   A

Untracked files:
3	B

",
        )
        .assert();
});

test!(dont_create_cache_file_without_repo, |mut t: Test| {
    t.gitnu("", "status").shell("", "ls -lA").expect_stdout("total 0\n");
});

test!(many_files, |mut t: Test| {
    use crate::data::LONG_EXPECT_NO_FLAG;
    t.shell(
        "",
        &(1..1000)
            .map(|v| format!(" {:0width$}", v, width = 5))
            .fold(String::from("touch"), |a, v| a + &v),
    )
    .gitnu("", "init")
    .gitnu("", "status")
    .gitnu("", "add 69-420")
    .gitnu("", "status")
    .expect_stderr("")
    .expect_stdout(LONG_EXPECT_NO_FLAG);
});

test!(support_aliases, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch X Y Z 3")
        .gitnu("", "status")
        .gitnu("", "config alias.scroll reflog")
        .gitnu("", "config alias.dank reset")
        .gitnu("", "config alias.memes add")
        // if aliases aren't supported then gitnu will not convert
        // the number 2 to a filename
        .gitnu("", "memes 3")
        .gitnu("", "status")
        // here, `Y` is added instead of `3`
        .expect_stdout(
            "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   Y

Untracked files:
2	3
3	X
4	Z

",
        );
});

test!(stop_after_double_dash, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch 3 fileA fileB")
        .gitnu("", "status")
        .expect_stdout(
            "
---
On branch main

No commits yet

Untracked files:
1	3
2	fileA
3	fileB

nothing added to commit but untracked files present
",
        )
        .assert()
        // if gitnu continues parsing after double dash then
        // fileB will be added instead
        .gitnu("", "add 2 -- 3")
        .gitnu("", "status")
        .expect_stdout(
            "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   3
2	new file:   fileA

Untracked files:
3	fileB

",
        )
        .assert();
});

test!(every_git_state, |mut t: Test| {
    t.gitnu("", "init");
    for file in "A B C D E F G H I".split(' ') {
        t.shell("", &format!("touch {}", file));
        t.write_to_file(file, file);
    }
    t.gitnu("", "add B C D E G H I")
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
});

test!(merge_conflict, |mut t: Test| {
    t.gitnu("", "init")
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
});

test!(detached_head, |mut t: Test| {
    t.gitnu("", "init")
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
});

test!(skip_short_flags, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch A B C D E F G H I J")
        .gitnu("", "status");
    for f in "A B C D E F G H I J".split(' ') {
        t.gitnu("", &format!("add {}", f));
        t.gitnu("", &format!("commit -m commit::{}", f));
    }
    // don't parse the 5 after the -n flag because it's likely used as
    // a value to that flag
    //
    // note that 6-8 is still parsed because the flag before them is a
    // long flag
    t.gitnu("", "log -n 5 --pretty=%s 6-8").expect_stdout(
        "
---
commit::H
commit::G
commit::F
",
    );
});

test!(handle_capital_c_flag, |mut t: Test| {
    t.shell("", "mkdir one two")
        .shell("", "touch one/one_file two/two_file")
        // populate both repositories' cache
        .gitnu("one", "init")
        .gitnu("one", "status")
        .gitnu("two", "init")
        .gitnu("two", "status")
        // run commands from /two
        .gitnu("two", "-C ../one add 1")
        .gitnu("two", "-C ../one status")
        .expect_stdout(
            "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   one_file

",
        )
        .assert()
        .gitnu("two", "add 1")
        .gitnu("two", "status")
        .expect_stdout(
            "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   two_file

",
        )
        .assert();
});
