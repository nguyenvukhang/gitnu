use crate::tests::util::*;

macro_rules! check {
    ($test:expr, $received:expr, $expected:expr) => {
        gitnu!($test, status);
        let app = gitnu!($test, $received);
        assert_args!(&app, $expected);
    };
}

test!(git_add_untracked, |t: &Test| {
    sh!(t, "git init -b main");
    sh!(t, "touch A");

    // status appearance
    assert_eq_pretty!(
        sh!(t, "git status").stdout,
        "\
On branch main\n
No commits yet\n
Untracked files:
	A

nothing added to commit but untracked files present\n"
    );

    // status add check
    check!(t, ["add", "1"], ["git", "add", "A"]);
});

test!(git_add_modified, |t: &Test| {
    sh!(t, "git init -b main");
    sh!(t, "touch A");
    sh!(t, "git add --all && git commit -m 'first'");
    fs::write(t.dir.join("A"), b"content").unwrap();

    // status appearance
    assert_eq_pretty!(
        sh!(t, "git status").stdout,
        "\
On branch main
Changes not staged for commit:
	modified:   A

no changes added to commit\n"
    );

    // status add check
    check!(t, ["add", "1"], ["git", "add", "A"]);
});

test!(git_add_deleted, |t: &Test| {
    sh!(t, "git init -b main");
    sh!(t, "touch A");
    sh!(t, "git add --all && git commit -m 'first'");
    sh!(t, "rm A");

    // status appearance
    assert_eq_pretty!(
        sh!(t, "git status").stdout,
        "\
On branch main
Changes not staged for commit:
	deleted:    A

no changes added to commit\n"
    );

    // status add check
    check!(t, ["add", "1"], ["git", "add", "A"]);
});

test!(git_add_both_modified, |t: &Test| {
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

    // status appearance
    assert_eq_pretty!(
        sh!(t, "git status").stdout,
        "\
On branch RIGHT
You have unmerged paths.\n
Unmerged paths:
	both added:      conflict_file

no changes added to commit\n"
    );

    // status add check
    check!(t, ["add", "1"], ["git", "add", "conflict_file"]);
});
