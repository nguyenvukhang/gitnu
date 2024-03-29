#[macro_use]
mod macros;

mod status_display;

#[test]
fn staging_files_with_numbers_manual() {}

// staging files with numbers
test!(
    staging_files_with_numbers,
    |t| {
        sh!(t, "git init -b main");
        sh!(t, "touch A B C D E F G");
        gitnu!(t, status).unwrap();
    },
    ["add", "2-4", "6"],
    ["add", "B", "C", "D", "F"]
);

// This just tests that the cache can be read more than once.
// Possible idea: make the cache readable only once.
// (If a number is called again, just insert nothing.)
test!(
    range_overlap,
    |t| {
        sh!(t, "git init -b main");
        sh!(t, "touch A B C D E F");
        gitnu!(t, status).unwrap();
    },
    ["add", "2-4", "3-5"],
    ["add", "B", "C", "D", "C", "D", "E"]
);

// Unindexed numbers will appear as the number itself, since it does
// not correspond to any file.
test!(
    add_unindexed_number,
    |t| {
        sh!(t, "git init -b main");
        sh!(t, "touch A B C");
        gitnu!(t, status).unwrap();
    },
    ["add", "2-5"],
    ["add", "B", "C", "4", "5"]
);

// Both `gitnu status` and `gitnu add <files>` are ran from the same
// directory, but that directory is not the workspace root
test!(
    not_at_workspace_root,
    |t| {
        sh!(t, "git init -b main");
        sh!(t, "mkdir src");
        sh!(t, "touch A B src/C src/D");
        gitnu!(t, "src", status).unwrap();
    },
    "src",
    ["add", "2", "3"],
    ["add", "../B", "./"]
);

// `gitnu status` is ran from a different directory than
// `gitnu add <files>`
test!(
    add_and_status_diff_dirs,
    |t| {
        // `gitnu status` will be ran from <root>, and
        // `gitnu add` will be ran from <root>/src
        sh!(t, "git init -b main");
        sh!(t, "mkdir src");
        sh!(t, "touch A B src/C src/D");
        gitnu!(t, status).unwrap();
    },
    "src",
    ["add", "2", "3"],
    ["add", "../B", "../src/"]
);

// If `git-nu` is ran in a directory that is not in a git
// workspace, then do not create the cache file.
test!(dont_create_cache_file_without_repo, |t| {
    use crate::prelude::*;

    let parsed = gitnu!(t, status);
    assert!(parsed.is_err());
    assert_eq!(parsed.as_ref().err(), Some(&Error::NotGitRepository));
    assert_eq!(sh!(t, "ls -lA").stdout.trim(), "total 0");
});

// Determined in ../git_cmd.rs
// where it's specified if a command should be skipped because it might
// be part of a flag value
test!(
    skip_flags,
    |t| {
        sh!(t, "git init -b main");
        sh!(t, "touch A B C");
        gitnu!(t, status).unwrap();
    },
    ["log", "-n", "2", "--oneline", "3"],
    ["log", "-n", "2", "--oneline", "C"]
);

// Running git reset with a number will make git-nu take the one on
// the right.
//
// ```
//   On branch main
//   Changes to be committed:
//   1       renamed:    A -> C
// ```
//
// `C` is chosen to replace `1` because the file C actually exists.
//
// Moreover, running `git reset A` (and not git nu reset A) will give
// the error "fatal: ambiguous argument 'A': unknown revision or..."
// anyway
test!(
    renames,
    |t| {
        sh!(t, "git init -b main");
        sh!(t, "touch A && git add A && git commit -m x");
        sh!(t, "mv A B && git add --all");
        sh!(t, "git nu status");
        gitnu!(t, status).unwrap();
    },
    ["add", "1"],
    ["add", "B"]
);

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

// git aliases
test!(aliases, |t| {
    sh!(t, "git init -b main");
    sh!(t, "touch A && git add A && git commit -m 'first'");
    sh!(t, "git config --global alias.teststatus status");
    let status = sh!(t, "git teststatus");
    sh!(t, "git config --global --unset alias.teststatus");
    assert_eq!(
        status.stdout,
        "\
On branch main
nothing to commit, working tree clean\n"
    );
});
