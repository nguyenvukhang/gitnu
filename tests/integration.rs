use crate::status;

gitnu_test!(untracked_files, |mut t: Test| {
    t.gitnu("", "init").shell("", "touch file:1 file_2 file-3");
    status::normal!(
        t,
        "
---
On branch main

No commits yet

Untracked files:
1	file-3
2	file:1
3	file_2

nothing added to commit but untracked files present
"
    );
    status::short!(
        t,
        "
---
1  ?? file-3
2  ?? file:1
3  ?? file_2
"
    );
});

gitnu_test!(staging_files_with_filename, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch file:1 file_2 file-3")
        .gitnu("", "add file_2"); // use filename
    status::normal!(
        t,
        "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   file_2

Untracked files:
2	file-3
3	file:1

"
    );
    status::short!(
        t,
        "
---
1  A  file_2
2  ?? file-3
3  ?? file:1
"
    );
});

gitnu_test!(staging_files_with_numbers, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch A B C D E F G")
        .gitnu("", "status")
        .gitnu("", "add 2-4 6"); // use number and range
    status::normal!(
        t,
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

"
    );
    status::short!(
        t,
        "
---
1  A  B
2  A  C
3  A  D
4  A  F
5  ?? A
6  ?? E
7  ?? G
"
    );
});

gitnu_test!(range_overlap, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch A B C D E F G")
        .gitnu("", "status")
        .gitnu("", "add 1-4 2-6");
    status::normal!(
        t,
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

"
    );
    status::short!(
        t,
        "
---
1  A  A
2  A  B
3  A  C
4  A  D
5  A  E
6  A  F
7  ?? G
"
    );
});

// Just as git would respond, no files will be added at all
gitnu_test!(add_unindexed_number, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch A B C")
        .gitnu("", "status")
        .gitnu("", "add 2-5"); // 2-5 out of range
    status::normal!(
        t,
        "
---
On branch main

No commits yet

Untracked files:
1	A
2	B
3	C

nothing added to commit but untracked files present
"
    );
    status::short!(
        t,
        "
---
1  ?? A
2  ?? B
3  ?? C
"
    );
});

// Just as git would respond, some files will be reset
gitnu_test!(reset_unindexed_number, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch A B C")
        .gitnu("", "add --all")
        .gitnu("", "status")
        .gitnu("", "reset 2-5"); // 2-5 out of range
    status::normal!(
        t,
        "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   A

Untracked files:
2	B
3	C

"
    );
    status::short!(
        t,
        "
---
1  A  A
2  ?? B
3  ?? C
"
    );
});

gitnu_test!(not_from_git_root, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "mkdir src")
        .shell("", "touch file1 file2 file3 src/.gitkeep")
        .gitnu("", "add src")
        .gitnu("", "commit -m src_dir")
        .shell("src", "touch emerald sapphire ruby")
        .gitnu("src", "status")
        .gitnu("src", "add 3-5");
    status::normal!(
        t,
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

"
    );
    status::short!(
        t,
        "
---
1  A  file3
2  A  src/emerald
3  A  src/ruby
4  ?? file1
5  ?? file2
6  ?? src/sapphire
"
    );
});

gitnu_test!(change_cwd_after_status, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "mkdir src")
        .shell("", "touch file1 file2 file3 src/.gitkeep")
        .gitnu("", "add src")
        .gitnu("", "commit -m src_dir")
        .shell("src", "touch emerald sapphire ruby")
        .gitnu("src", "status") // run status command in /src
        .gitnu("", "add 3-5"); // run add command from /
    status::normal!(
        t,
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

"
    );
    status::short!(
        t,
        "
---
1  A  file3
2  A  src/emerald
3  A  src/ruby
4  ?? file1
5  ?? file2
6  ?? src/sapphire
"
    );
});

gitnu_test!(zero_handling, |mut t: Test| {
    t.gitnu("", "init").shell("", "touch A B");
    let normal = "
---
On branch main

No commits yet

Untracked files:
1	A
2	B

nothing added to commit but untracked files present
";
    let short = "
---
1  ?? A
2  ?? B
";
    t.gitnu("", "add 0");
    status::normal!(t, normal);
    status::short!(t, short);
    t.gitnu("", "add 0-1");
    status::normal!(t, normal);
    status::short!(t, short);
    t.gitnu("", "add 0-0");
    status::normal!(t, normal);
    status::short!(t, short);
});

gitnu_test!(zero_filename, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch 0 A B")
        .gitnu("", "status")
        // expand `gitnu add 0-2` to `git add 0 1 2`
        // and then convert it to filenames: `git add 0 0 A`
        // therefore only adding files `0` and `A`
        .gitnu("", "add 0-2");
    status::normal!(
        t,
        "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   0
2	new file:   A

Untracked files:
3	B

"
    );
    status::short!(
        t,
        "
---
1  A  0
2  A  A
3  ?? B
"
    );
});

gitnu_test!(dont_create_cache_file_without_repo, |mut t: Test| {
    t.gitnu("", "status");
    status::general!(t, "ls -lA", "total 0\n");
});

gitnu_test!(many_files, |mut t: Test| {
    use crate::data::{LONG_EXPECT_NO_FLAG, LONG_EXPECT_SHORT_FLAG};
    t.shell(
        "",
        &(1..1000)
            .map(|v| format!(" {:0width$}", v, width = 5))
            .fold(String::from("touch"), |a, v| a + &v),
    )
    .gitnu("", "init")
    .gitnu("", "status")
    .gitnu("", "add 69-420");
    status::normal!(t, LONG_EXPECT_NO_FLAG);
    status::short!(t, LONG_EXPECT_SHORT_FLAG);
});

gitnu_test!(support_aliases, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch X Y Z 3")
        .gitnu("", "status")
        .gitnu("", "config alias.scroll reflog")
        .gitnu("", "config alias.dank reset")
        .gitnu("", "config alias.memes add")
        // if aliases aren't supported then gitnu will not convert
        // the number 2 to a filename
        .gitnu("", "memes 3");
    // here, `Y` is added instead of `3`
    status::normal!(
        t,
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

"
    );
    status::short!(
        t,
        "
---
1  A  Y
2  ?? 3
3  ?? X
4  ?? Z
"
    );
});

gitnu_test!(stop_after_double_dash, |mut t: Test| {
    t.gitnu("", "init").shell("", "touch 3 fileA fileB").gitnu("", "status");
    // if gitnu continues parsing after double dash then
    // fileB will be added instead
    t.gitnu("", "add 2 -- 3");
    status::normal!(
        t,
        "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   3
2	new file:   fileA

Untracked files:
3	fileB

"
    );
    status::short!(
        t,
        "
---
1  A  3
2  A  fileA
3  ?? fileB
"
    );
});

gitnu_test!(every_git_state, |mut t: Test| {
    t.gitnu("", "init");
    for file in "A B C D E F G H I".split(' ') {
        t.shell("", &format!("touch {}", file));
        t.write_file(file, file);
    }
    t.gitnu("", "add B C D E G H I")
        .gitnu("", "commit -m pre")
        // modify
        .write_file("B", "B_")
        .write_file("G", "G_")
        // remove
        .remove("C")
        // rename
        .rename("E", "_E")
        .rename("I", "_I")
        // typechange
        .shell("", "ln -sf . D")
        .shell("", "ln -sf . H")
        .gitnu("", "add A B C D E _E");
    status::normal!(
        t,
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

"
    );
    status::short!(
        t,
        "
---
1  A  A
2  M  B
3  D  C
4  T  D
5   M G
6   T H
7   D I
8  R  E -> _E
9  ?? F
10 ?? _I
"
    );
});

gitnu_test!(merge_conflict, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch base")
        .gitnu("", "add --all")
        .gitnu("", "commit -m base_commit")
        // left branch
        .gitnu("", "branch -m LEFT")
        .shell("", "touch conflict_file")
        .write_file("conflict_file", "left")
        .gitnu("", "add --all")
        .gitnu("", "commit -m left_commit")
        // right branch
        .gitnu("", "checkout -b RIGHT")
        .gitnu("", "reset --hard HEAD~1")
        .shell("", "touch conflict_file")
        .write_file("conflict_file", "right")
        .gitnu("", "add --all")
        .gitnu("", "commit -m right_commit")
        // merge
        .gitnu("", "merge LEFT")
        .shell("", "touch fileA fileB fileC")
        .gitnu("", "add fileA")
        .gitnu("", "status")
        .gitnu("", "add 3");
    status::normal!(
        t,
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

"
    );
    status::short!(
        t,
        "
---
1  AA conflict_file
2  A  fileA
3  A  fileB
4  ?? fileC
"
    );
});

gitnu_test!(detached_head, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch file1")
        .gitnu("", "add --all")
        .gitnu("", "commit -m add::file1")
        .shell("", "touch file2")
        .gitnu("", "add --all")
        .gitnu("", "commit -m add::file2")
        .gitnu("", "checkout HEAD~1")
        .set_sha();
    status::normal!(
        t,
        "
---
HEAD detached at [:SHA:]
nothing to commit, working tree clean
"
    );
    status::short!(t, "");
});

gitnu_test!(skip_short_flags, |mut t: Test| {
    t.gitnu("", "init")
        .shell("", "touch A B C D E F G H I J")
        .gitnu("", "status");
    for f in "A B C D E F G H I J".split(' ') {
        t.gitnu("", &format!("add {}", f));
        t.gitnu("", &format!("commit -m commit_{}", f));
    }
    // don't parse the 5 after the -n flag because it's likely used as
    // a value to that flag
    //
    // note that 6-8 is still parsed because the flag before them is a
    // long flag
    status::general!(
        t,
        "gitnu log -n 5 --pretty=%s 6-8",
        "
---
commit_H
commit_G
commit_F
"
    );
});

gitnu_test!(handle_capital_c_flag, |mut t: Test| {
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
        .gitnu("two", "add 1");
    status::normal!(
        t,
        "one",
        "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   one_file

"
    );
    status::normal!(
        t,
        "two",
        "
---
On branch main

No commits yet

Changes to be committed:
1	new file:   two_file

"
    );
    status::short!(
        t,
        "one",
        "
---
1  A  one_file
"
    );
    status::short!(
        t,
        "two",
        "
---
1  A  two_file
"
    );
});
