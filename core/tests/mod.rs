#[macro_use]
mod util;
mod status_parse;

use util::*;

// staging files with numbers
test!(
    staging_files_with_numbers,
    |t| {
        sh!(t, "git init -b main");
        sh!(t, "touch A B C D E F G");
        gitnu!(t, status);
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
        gitnu!(t, status);
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
        gitnu!(t, status);
    },
    ["add", "2-5"],
    ["add", "B", "C", "4", "5"]
);

// Both `gitnu status` and `gitnu add <files>` are ran from the same
// directory, but that directory is not the workspace root
test!(not_at_workspace_root, |t| {
    sh!(t, "git init -b main");
    sh!(t, "mkdir src");
    sh!(t, "touch A B src/C src/D");
    gitnu!(t, "src", ["status"]).run().unwrap();
    let app = gitnu!(t, "src", ["add", "2", "3"]);
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
test!(
    skip_flags,
    |t| {
        sh!(t, "git init -b main");
        sh!(t, "touch A B C");
        gitnu!(t, status);
    },
    ["log", "-n", "2", "--oneline", "3"],
    ["log", "-n", "2", "--oneline", "C"]
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

// git reset --hard
test!(
    reset_hard_on_numeric_sha,
    |t| {
        sh!(t, "git init -b 1234567");
        sh!(t, "touch A && git add A && git commit -m 'first'");
        sh!(t, "git checkout -b main");
        sh!(t, "touch B && git add B && git commit -m 'second'");
        sh!(t, "git branch");
        sh!(t, "git nu reset --hard 1234567");
    },
    ["reset", "--hard", "1234567"],
    ["reset", "--hard", "1234567"]
);

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
