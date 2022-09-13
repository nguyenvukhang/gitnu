mod utils;
use utils::{multiline, Git};

#[test]
fn untracked_file() {
    let (mut git, mut gitnu) = Git::gpair("tmp/g1");
    {
        git.init();
        git.touch("gold");
    }
    let received = gitnu.status();
    let expected = multiline(&[
        "On branch main\n",
        "No commits yet\n",
        "Untracked files:",
        "1\t\u{1b}[31mgold\u{1b}[m\n",
        "nothing added to commit but untracked files present\n",
    ]);
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
    let expected = multiline(&[
        "On branch main\n",
        "No commits yet\n",
        "Changes to be committed:",
        "1\t\u{1b}[32mnew file:   gold\u{1b}[m\n\n",
    ]);
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
    let expected = multiline(&[
        "On branch main\n",
        "No commits yet\n",
        "Changes to be committed:",
        "1\t\u{1b}[32mnew file:   gold\u{1b}[m\n\n",
    ]);
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
    let expected = multiline(&[
        "On branch main",
        "nothing to commit, working tree clean\n",
    ]);
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
    let expected = multiline(&[
        "On branch main",
        "nothing to commit, working tree clean\n",
    ]);
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
    let expected = multiline(&[
        "On branch main\n",
        "No commits yet\n",
        "Changes to be committed:",
        "1\t\u{1b}[32mnew file:   file_2\u{1b}[m",
        "2\t\u{1b}[32mnew file:   file_3\u{1b}[m",
        "3\t\u{1b}[32mnew file:   file_4\u{1b}[m",
        "4\t\u{1b}[32mnew file:   file_5\u{1b}[m\n",
        "Untracked files:",
        "5\t\u{1b}[31mfile_1\u{1b}[m",
        "6\t\u{1b}[31mfile_6\u{1b}[m",
        "7\t\u{1b}[31mfile_7\u{1b}[m\n\n",
    ]);
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
    let expected = multiline(&[
        "On branch main\n",
        "No commits yet\n",
        "Changes to be committed:",
        "1\t\u{1b}[32mnew file:   file_2\u{1b}[m",
        "2\t\u{1b}[32mnew file:   file_3\u{1b}[m\n",
        "Untracked files:",
        "3\t\u{1b}[31mfile_1\u{1b}[m",
        "4\t\u{1b}[31mfile_4\u{1b}[m",
        "5\t\u{1b}[31mfile_5\u{1b}[m",
        "6\t\u{1b}[31mfile_6\u{1b}[m",
        "7\t\u{1b}[31mfile_7\u{1b}[m\n\n",
    ]);
    assert_eq!(gitnu.status(), expected);
}
