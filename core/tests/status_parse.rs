use super::util::*;

macro_rules! status_test {
    ($name:ident, $setup:expr, $inout:expr, $status:expr) => {
        test!($name, |t| {
            let setup: Box<dyn Fn(&Test) -> ()> = Box::new($setup);
            setup(t);
            assert_eq!(sh!(t, "git status").stdout, $status);
            gitnu!(t, status);
            let (input, output) = $inout;
            assert_args!(gitnu!(t, input), output);
        });
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
	A

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
	modified:   A

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
	deleted:    A

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
	both added:      conflict_file

no changes added to commit\n"
);
