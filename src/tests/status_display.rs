macro_rules! status_test {
    ($name:ident, $setup:expr, $inout:expr, $status:expr) => {
        test!($name, |t| {
            let setup: Box<dyn Fn(&Test) -> ()> = Box::new($setup);
            setup(t);

            // insert the actual sha where there is "{GIT_SHA}"
            let status = if $status.contains("{GIT_SHA}") {
                let sha = sh!(t, "git rev-parse --short HEAD");
                $status.replace("{GIT_SHA}", sha.stdout.trim())
            } else {
                $status.to_string()
            };

            assert_eq!(sh!(t, "git nu status").stdout, status);

            gitnu!(t, status).unwrap();

            let (input, output) = $inout;
            let app = gitnu!(t, input).unwrap();
            assert_args!(app, output);
        });
    };
    ($name:ident, $setup:expr, $status:expr) => {
        status_test!($name, $setup, ([""], [""]), $status);
    };
}

status_test!(
    git_add_untracked,
    |t| {
        sh!(t, "git init -b main");
        sh!(t, "touch A");
    },
    (["add", "1"], ["add", "A"]),
    "\
On branch main\n
No commits yet\n
Untracked files:
1	A

nothing added to commit but untracked files present\n"
);

status_test!(
    git_add_modified,
    |t| {
        sh!(t, "git init -b main");
        sh!(t, "touch A");
        sh!(t, "git add A && git commit -m x");
        fs::write(t.dir.join("A"), b"content").unwrap();
    },
    (["add", "1"], ["add", "A"]),
    "\
On branch main
Changes not staged for commit:
1	modified:   A

no changes added to commit\n"
);

status_test!(
    git_add_deleted,
    |t| {
        sh!(t, "git init -b main");
        sh!(t, "touch A");
        sh!(t, "git add A && git commit -m x");
        sh!(t, "rm A");
    },
    (["add", "1"], ["add", "A"]),
    "\
On branch main
Changes not staged for commit:
1	deleted:    A

no changes added to commit\n"
);

status_test!(
    git_add_both_modified,
    |t| {
        // create base commit
        sh!(t, "git init -b main");
        sh!(t, "touch A && git add A && git commit -m x");

        // the conflict file
        let basename = "conflict_file";
        let filepath = t.dir.join(basename);

        // left branch
        sh!(t, "git branch -m LEFT");
        fs::write(&filepath, b"LEFT").unwrap();
        sh!(t, format!("git add {basename}"));
        sh!(t, "git commit -m x");

        // right branch
        sh!(t, "git checkout -b RIGHT");
        sh!(t, "git reset --hard HEAD~1");
        fs::write(&filepath, b"RIGHT").unwrap();
        sh!(t, format!("git add {basename}"));
        sh!(t, "git commit -m x");

        // merge and create the conflict
        sh!(t, "git merge LEFT");
    },
    (["add", "1"], ["add", "conflict_file"]),
    "\
On branch RIGHT
You have unmerged paths.

Unmerged paths:
1	both added:      conflict_file

no changes added to commit\n"
);

// This aims to cover every reachable case in one `gitnu status`
// Note that this doesn't cover:
//   1. merge conflict status
//   2. detached head status
//
// set to only unix since there are std::os::unix is used for symlinks
#[cfg(unix)]
status_test!(
    everything,
    |t| {
        use std::os::unix;
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
    },
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
10	_I\n\n"
);

// Special display case 1 of 2: Merge conflict
status_test!(
    merge_conflict,
    |t| {
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
        gitnu!(t, status).unwrap();

        gitnu!(t, ["add", "2"]).and_then(|v| v.run()).unwrap();
    },
    "\
On branch RIGHT
All conflicts fixed but you are still merging.

Changes to be committed:
1	modified:   conflict_file
2	new file:   fresh\n\n"
);

// Special display case 2 of 2: Detached Head
status_test!(
    detached_head,
    |t| {
        sh!(t, "git init -b main");
        sh!(t, "touch A && git add A && git commit -m 'A'");
        sh!(t, "touch B && git add B && git commit -m 'B'");
        sh!(t, "git checkout HEAD~1");
        sh!(t, "touch gold silver");
    },
    "\
HEAD detached at {GIT_SHA}
Untracked files:
1	gold
2	silver

nothing added to commit but untracked files present\n"
);

status_test!(
    max_cache_size_exceeded,
    |t| {
        sh!(t, "git init -b main");

        sh!(t, {
            let mut args = "touch".to_string();
            (1..25).for_each(|i| args += &format!(" f{i:0>2}"));
            args
        });

        gitnu!(t, status).unwrap();
    },
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

nothing added to commit but untracked files present\n"
);
