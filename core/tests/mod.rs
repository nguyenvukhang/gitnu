#[macro_use]
mod util;
mod status_parse;

use std::os::unix;
use util::*;

// staging files with numbers
test!(staging_files_with_numbers, |t| {
    sh!(t, "git init -b main");
    sh!(t, "touch A B C D E F G");
    gitnu!(t, status);
    let output = sh!(t, "ls -A .git");
    println!("output: {output:?}");
    let app = gitnu!(t, ["add", "2-4", "6"]);
    assert_args!(&app, ["add", "B", "C", "D", "F"]);
});

// This just tests that the cache can be read more than once.
// Possible idea: make the cache readable only once.
// (If a number is called again, just insert nothing.)
test!(range_overlap, |t| {
    sh!(t, "git init -b main");
    sh!(t, "touch A B C D E F");
    gitnu!(t, status);
    let app = gitnu!(t, ["add", "2-4", "3-5"]);
    assert_args!(&app, ["add", "B", "C", "D", "C", "D", "E"]);
});

// Unindexed numbers will appear as the number itself, since it does
// not correspond to any file.
test!(add_unindexed_number, |t| {
    sh!(t, "git init -b main");
    sh!(t, "touch A B C");
    gitnu!(t, status);
    let app = gitnu!(t, ["add", "2-5"]);
    assert_args!(&app, ["add", "B", "C", "4", "5"]);
});

// Both `gitnu status` and `gitnu add <files>` are ran from the same
// directory, but that directory is not the workspace root
test!(not_at_workspace_root, |t| {
    sh!(t, "git init -b main");
    sh!(t, "mkdir src");
    sh!(t, "touch A B src/C src/D");
    gitnu!(t, "src", ["status"]).run().ok();
    let app = gitnu!(t, "src", ["add", "2", "3"]);
    sh!(t, "cat .git/gitnu.txt");
    assert_args!(&app, ["add", "../B", "./"]);
});

// `gitnu status` is ran from a different directory than
// `gitnu add <files>`
test!(add_and_status_diff_dirs, |t| {
    // `gitnu status` will be ran from <root>, and
    // `gitnu add` will be ran from <root>/src
    sh!(t, "git init -b main");
    sh!(t, "mkdir src");
    sh!(t, "touch A B src/C src/D");
    gitnu!(t, status);
    let app = gitnu!(t, "src", ["add", "2", "3"]);
    assert_args!(&app, ["add", "../B", "../src/"]);
});

// If `gitnu status` is ran in a directory that is not in a git
// workspace, then do not create the cache file.
test!(dont_create_cache_file_without_repo, |t| {
    gitnu!(t, status);
    assert_eq!(sh!(t, "ls -lA").stdout.trim(), "total 0");
});

// Determined in ../git_cmd.rs
// where it's specified if a command should be skipped because it might
// be part of a flag value
test!(skip_flags, |t| {
    sh!(t, "git init -b main");
    sh!(t, "touch A B C");
    gitnu!(t, status);
    let app = gitnu!(t, ["log", "-n", "2", "--oneline", "3"]);
    assert_args!(&app, ["log", "-n", "2", "--oneline", "C"]);
});

// This aims to cover every reachable case in one `gitnu status`
// Note that this doesn't cover:
//   1. merge conflict status
//   2. detached head status
test!(status_display, |t| {
    sh!(t, "git init -b main");
    for file in "A B C D E F G H I".split(' ') {
        write(t, file, &format!("contents::{file}"));
    }
    sh!(t, "git add B C D E G H I");
    sh!(t, "git commit -m pre");
    // modify B and G
    fs::write(t.dir.join("B"), b"modify::B").unwrap();
    fs::write(t.dir.join("G"), b"modify::G").unwrap();
    // remove C
    fs::remove_file(t.dir.join("C")).unwrap();
    // rename E and I
    fs::rename(t.dir.join("E"), t.dir.join("_E")).unwrap();
    fs::rename(t.dir.join("I"), t.dir.join("_I")).unwrap();
    // tyepchange D and H
    fs::remove_file(t.dir.join("D")).unwrap();
    fs::remove_file(t.dir.join("H")).unwrap();
    unix::fs::symlink(t.dir.join("A"), t.dir.join("D")).unwrap();
    unix::fs::symlink(t.dir.join("A"), t.dir.join("H")).unwrap();
    // stage about half of the changes
    sh!(t, "git add A B C D E _E");
    let output = sh!(t, format!("git nu status"));
    assert_eq!(
        output.stdout,
        "\
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
});

// Special display case 1 of 2: Merge conflict
test!(merge_conflict_display, |t| {
    // create base commit
    sh!(t, "git init -b main");
    sh!(t, "touch base");
    sh!(t, "git add --all");
    sh!(t, "git commit -m 'base commit'");

    // left branch
    sh!(t, "git branch -m LEFT");
    fs::write(t.dir.join("conflict_file"), b"left").unwrap();
    sh!(t, "git add conflict_file");
    sh!(t, "git commit -m 'left commit'");

    // right branch
    sh!(t, "git checkout -b RIGHT");
    sh!(t, "git reset --hard HEAD~1");
    fs::write(t.dir.join("conflict_file"), b"right").unwrap();
    sh!(t, "git add conflict_file");
    sh!(t, "git commit -m 'right commit'");

    // merge
    sh!(t, "git merge LEFT");
    sh!(t, "touch fresh");
    sh!(t, "git add fresh");
    gitnu!(t, status);
    gitnu!(t, ["add", "2"]).run().ok();

    let output = sh!(t, format!("git nu status"));
    assert_eq!(
        output.stdout,
        "\
On branch RIGHT
All conflicts fixed but you are still merging.

Changes to be committed:
1	modified:   conflict_file
2	new file:   fresh

"
    );
});

// Special display case 2 of 2: Detached Head
test!(detached_head_display, |t| {
    sh!(t, "git init -b main");
    sh!(t, "touch A && git add A && git commit -m 'A'");
    sh!(t, "touch B && git add B && git commit -m 'B'");
    sh!(t, "git checkout HEAD~1");
    sh!(t, "touch gold silver");
    let sha = sh!(t, "git rev-parse --short HEAD");
    let sha = sha.stdout.trim();

    let output = sh!(t, format!("git nu status"));
    assert_eq!(
        output.stdout,
        format!(
            "\
HEAD detached at {sha}
Untracked files:
1	gold
2	silver

nothing added to commit but untracked files present
"
        )
    );
});

// Ensure that `gitnu` exit codes match those of `git`. This means
// that error handling bubbles up properly.
test!(exit_codes, |t| {
    macro_rules! assert_code {
        ($cmd:expr, $code:expr) => {
            assert_eq!(sh!(t, $cmd).exit_code, Some($code));
        };
    }
    // ran outside of a repository
    assert_code!("git status", 128);
    assert_code!("git nu status", 128);

    assert_code!("git status --bad-flag", 128);
    assert_code!("git nu status --bad-flag", 128);

    assert_code!("git stat", 1);
    assert_code!("git nu stat", 1);

    // ran inside of a repository
    sh!(t, "git init -b main");

    assert_code!("git status", 0);
    assert_code!("git nu status", 0);
});

// Run `gitnu` from a different repository using the `-C` flag.
test!(different_workspace, |t| {
    sh!(t, "mkdir one two");
    sh!(t, "one", "git init -b one");
    sh!(t, "two", "git init -b two");
    sh!(t, "one", "touch gold silver");
    sh!(t, "two", "git -C ../one nu status");
    sh!(t, "two", "git -C ../one nu add 1");

    let status = sh!(t, "two", "git -C ../one nu status");
    assert_eq!(
        status.stdout,
        "\
On branch one

No commits yet

Changes to be committed:
1	new file:   gold

Untracked files:
2	silver

"
    );
});

// staging files with numbers
test!(max_cache_size_exceeded, |t| {
    sh!(t, "git init -b main");
    let mut touch_args = "touch".to_string();
    (1..25).for_each(|i| touch_args += &format!(" f{i:0>2}"));
    sh!(t, &touch_args);
    gitnu!(t, status);
    let output = sh!(t, "git nu status");
    assert_eq!(
        output.stdout,
        "\
        On branch main

No commits yet

Untracked files:
1	f01
2	f02
3	f03
4	f04
5	f05
6	f06
7	f07
8	f08
9	f09
10	f10
11	f11
12	f12
13	f13
14	f14
15	f15
16	f16
17	f17
18	f18
19	f19
20	f20
	f21
	f22
	f23
	f24

nothing added to commit but untracked files present
"
    );
});

// git reset --hard
test!(reset_hard_on_numeric_sha, |t| {
    sh!(t, "git init -b 1234567");
    sh!(t, "touch A && git add A && git commit -m 'first'");
    sh!(t, "git checkout -b main");
    sh!(t, "touch B && git add B && git commit -m 'second'");
    sh!(t, "git branch");
    sh!(t, "git nu reset --hard 1234567");
    let app = gitnu!(t, ["reset", "--hard", "1234567"]);
    assert_args!(&app, ["reset", "--hard", "1234567"]);
});
