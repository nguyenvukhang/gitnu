mod utils;
use utils::{trimf, Git};

#[test]
fn untracked_file() {
    let (mut git, mut gitnu) = Git::gpair("tmp/g1");
    {
        git.init();
        git.touch("gold");
    }
    let received = gitnu.status();
    let expected = trimf(
        "
On branch main

No commits yet

Untracked files:
1\t\u{1b}[31mgold\u{1b}[m

nothing added to commit but untracked files present
",
    );
    assert_eq!(received, expected);
}

#[test]
fn staged_file_by_name() {
    let (mut git, mut gitnu) = Git::gpair("tmp/g2");
    {
        git.init();
        git.touch("gold");
        gitnu.add("gold");
    }
    let expected = trimf(
        "
On branch main

No commits yet

Changes to be committed:
1\t\u{1b}[32mnew file:   gold\u{1b}[m

",
    );

    assert_eq!(gitnu.status(), expected);
}

#[test]
fn staged_file_by_number() {
    let (mut git, mut gitnu) = Git::gpair("tmp/g3");
    {
        git.init();
        git.touch("gold");
        gitnu.status();
        gitnu.add("1");
    }
    let expected = trimf(
        "
On branch main

No commits yet

Changes to be committed:
1\t\u{1b}[32mnew file:   gold\u{1b}[m

",
    );
    assert_eq!(gitnu.status(), expected);
}

#[test]
fn modified_file_by_name() {
    let (mut git, mut gitnu) = Git::gpair("tmp/g4");
    {
        git.init();
        git.touch("gold");
        gitnu.add("gold");
        gitnu.commit();
    }
    let expected = trimf(
        "
On branch main
nothing to commit, working tree clean
",
    );
    assert_eq!(gitnu.status(), expected);
}

#[test]
fn modified_file_by_number() {
    let (mut git, mut gitnu) = Git::gpair("tmp/g5");
    {
        git.init();
        git.touch("gold");
        gitnu.status();
        gitnu.add("1");
        gitnu.commit();
    }
    let expected = trimf(
        "
On branch main
nothing to commit, working tree clean
",
    );
    assert_eq!(gitnu.status(), expected);
}

#[test]
fn untracked_file_by_range() {
    let (mut git, mut gitnu) = Git::gpair("tmp/g6");
    {
        git.init();
        for i in 1..8 {
            git.touch(&format!("file_{}", i));
        }
        gitnu.status();
        gitnu.add("2-5");
    }
    let expected = trimf(
        "
On branch main

No commits yet

Changes to be committed:
1\t\u{1b}[32mnew file:   file_2\u{1b}[m
2\t\u{1b}[32mnew file:   file_3\u{1b}[m
3\t\u{1b}[32mnew file:   file_4\u{1b}[m
4\t\u{1b}[32mnew file:   file_5\u{1b}[m

Untracked files:
5\t\u{1b}[31mfile_1\u{1b}[m
6\t\u{1b}[31mfile_6\u{1b}[m
7\t\u{1b}[31mfile_7\u{1b}[m

",
    );
    assert_eq!(gitnu.status(), expected);
}

#[test]
fn untracked_file_by_range_twice() {
    let (mut git, mut gitnu) = Git::gpair("tmp/g7");
    {
        git.init();
        for i in 1..8 {
            git.touch(&format!("file_{}", i));
        }
        gitnu.status();
        gitnu.add("2-5");
        gitnu.status();
        gitnu.reset("3-6");
    }
    let expected = trimf(
        "
On branch main

No commits yet

Changes to be committed:
1\t\u{1b}[32mnew file:   file_2\u{1b}[m
2\t\u{1b}[32mnew file:   file_3\u{1b}[m

Untracked files:
3\t\u{1b}[31mfile_1\u{1b}[m
4\t\u{1b}[31mfile_4\u{1b}[m
5\t\u{1b}[31mfile_5\u{1b}[m
6\t\u{1b}[31mfile_6\u{1b}[m
7\t\u{1b}[31mfile_7\u{1b}[m

",
    );
    assert_eq!(gitnu.status(), expected);
}
