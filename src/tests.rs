#![cfg(test)]
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::{env, fs};

use crate::parse;
use crate::prelude::*;
use crate::{main_cli, prefetch};

// A word on why it's necessary to have the debug `git-nu` binary built and
// prepended to $PATH...
//
// We want to run tests on shell calls such as
// $ git -C ../one nu status
//
// Which tests for how git processes pre-command args and mixed them with calls
// to git-nu.
//
// We also want to test for exit codes, which is much easier to test externally.

const TEST_DIR: &str = "gitnu-tests";

const EM: [&str; 0] = [];

macro_rules! color {
    ($($name:ident, $num:expr),+) => {
        pub trait Colored{$(fn $name(&self)->String;)*}
        impl<S:AsRef<str>>Colored for S{
            $(fn $name(&self)->String{format!("\x1b[0;{}m{}\x1b[0m",$num,self.as_ref())})*
        }
    };
}
color!(green, 32, yellow, 33, purple, 35, cyan, 36, gray, 37);

struct Test {
    dir: PathBuf,
}

impl Test {
    /// Run `git-nu` at a directory relative to the test root dir.
    fn gitnu<S, I, P>(&self, rel_dir: P, args: I) -> Result<ExitStatus>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
        P: AsRef<Path>,
    {
        let mut x = vec!["git".to_string()];
        x.extend(args.into_iter().map(|v| v.as_ref().to_string()));
        main_cli(self.dir.join(rel_dir), &x)
    }

    /// Parse `git-nu` args at a directory relative to the test root dir.
    fn gitnu_parse<S, I, P>(&self, rel_dir: P, args: I) -> Result<Vec<String>>
    where
        S: AsRef<str>,
        I: IntoIterator<Item = S>,
        P: AsRef<Path>,
    {
        let mut x = vec!["git".to_string()];
        x.extend(args.into_iter().map(|v| v.as_ref().to_string()));
        let cwd = self.dir.join(rel_dir);
        let (cwd, git_dir, git_aliases) = prefetch(cwd)?;
        let cache = Cache::new(&git_dir, &cwd);
        Ok(parse::parse(&x, git_aliases, cache, vec![]).0)
    }

    /// Run a shell command at a directory relative to the test root dir.
    fn sh<P, S>(&self, rel_dir: P, cmd: S) -> Output
    where
        P: AsRef<str>,
        S: AsRef<str>,
    {
        let (cmd, rel_dir) = (cmd.as_ref(), rel_dir.as_ref());
        let cmd = match cmd.starts_with("git") {
            false => cmd.to_string(),
            _ => cmd.replacen("git", "git -c advice.statusHints=false", 1),
        };
        let v = Command::new("sh")
            .current_dir(self.dir.join(rel_dir))
            .arg("-c")
            .arg(&cmd)
            .output()
            .unwrap();
        let root2 = if self.dir.is_absolute() { "/<root>" } else { "<root>" };
        let root = self.dir.as_os_str().to_str().unwrap();
        let (x, y) = ("[".gray(), "]".gray());
        println!("> {x}{} {} {}{y}", rel_dir.cyan(), "||".gray(), cmd.cyan());
        let stdout = String::from_utf8_lossy(&v.stdout).replace(root, root2);
        let stderr = String::from_utf8_lossy(&v.stderr).replace(root, root2);
        pretty_print("stdout", &stdout);
        pretty_print("stderr", &stderr);
        Output { stdout: stdout.to_string(), exit_code: v.status.code() }
    }
}

#[derive(Debug)]
#[allow(unused)]
struct Output {
    pub stdout: String,
    pub exit_code: Option<i32>,
}

/// Pretty-printer for stdout/stderr with fancy purple xml tags.
fn pretty_print<S: AsRef<str>>(tag: &str, output: S) {
    let output = output.as_ref();
    if !output.is_empty() {
        println!("{}{}{}", "<".gray(), tag.purple(), ">".gray());
        println!("{}", output.trim().gray());
        println!("{}{}{}", "</".gray(), tag.purple(), ">".gray());
    }
}

/// Get the path to the debug binary
fn bin_dir() -> String {
    let mut p = env::current_exe().unwrap();
    (p.pop(), p.pop(), p.to_string_lossy().trim().to_string()).2
}

/// Gets an environment variable with a maximum of 100 retries.
fn env_var(name: &str) -> String {
    let mut max_retries: usize = 100;
    let mut path = env::var(name).ok();
    loop {
        if max_retries == 0 {
            panic!("Exceeded max retries while trying to get env: {name}");
        }
        max_retries -= 1;
        match path {
            Some(v) if !v.trim_matches(char::from(0)).is_empty() => return v,
            _ => path = env::var(name).ok(),
        }
    }
}

/// 1. Clear and re-create the test directory
/// 2. Set the $PATH to ensure that the debug binary is front-and-center.
fn prep_test(name: &str) -> PathBuf {
    let test_dir = env::temp_dir().join(TEST_DIR).join(&name);
    test_dir.exists().then(|| fs::remove_dir_all(&test_dir));
    fs::create_dir_all(&test_dir).unwrap();

    // prepend debug bin directory to path.
    env::set_var("PATH", format!("{}:{}", bin_dir(), env_var("PATH")));

    test_dir
}

pub(crate) fn type_name_of<'a, T>(_: T) -> &'a str {
    std::any::type_name::<T>()
}

/// Runs the test in an isolated directory.
macro_rules! test {
    ($fn:ident, $run:expr) => {
        test!($fn, $run, "", EM, EM);
    };
    ($fn:ident, $run:expr, $input_args:expr, $output_args:expr) => {
        test!($fn, $run, "", $input_args, $output_args);
    };
    ($fn:ident, $run:expr, $rel_dir:expr, $input_args:expr, $output_args:expr) => {
        #[test]
        fn $fn() {
            fn f() {}
            let test_dir = prep_test(type_name_of(f));
            let run: Box<dyn Fn(&Test) -> ()> = Box::new($run);
            let t = Test { dir: test_dir.clone() };
            run(&t);
            if !$input_args.is_empty() {
                let rec = t.gitnu_parse($rel_dir, $input_args).unwrap();
                assert_eq!(rec, $output_args);
            }
            fs::remove_dir_all(&test_dir).ok();
        }
    };
}

test!(test_macro_works, |_| {});

// staging files with numbers
test!(
    staging_files_with_numbers,
    |t| {
        t.sh("", "git init -b main");
        t.sh("", "touch A B C D E F G");
        let _ = t.gitnu("", ["status"]);
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
        t.sh("", "git init -b main");
        t.sh("", "touch A B C D E F");
        let _ = t.gitnu("", ["status"]);
    },
    ["add", "2-4", "3-5"],
    ["add", "B", "C", "D", "C", "D", "E"]
);

// Unindexed numbers will appear as the number itself, since it does
// not correspond to any file.
test!(
    add_unindexed_number,
    |t| {
        t.sh("", "git init -b main");
        t.sh("", "touch A B C");
        let _ = t.gitnu("", ["status"]);
    },
    ["add", "2-5"],
    ["add", "B", "C", "4", "5"]
);

// Both `gitnu status` and `gitnu add <files>` are ran from the same
// directory, but that directory is not the workspace root
test!(
    not_at_workspace_root,
    |t| {
        t.sh("", "git init -b main");
        t.sh("", "mkdir src");
        t.sh("", "touch A B src/C src/D");
        let _ = t.gitnu("src", ["status"]);
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
        t.sh("", "git init -b main");
        t.sh("", "mkdir src");
        t.sh("", "touch A B src/C src/D");
        let _ = t.gitnu("", ["status"]);
    },
    "src",
    ["add", "2", "3"],
    ["add", "../B", "../src/"]
);

// If `git-nu` is ran in a directory that is not in a git
// workspace, then do not create the cache file.
test!(dont_create_cache_file_without_repo, |t| {
    use crate::prelude::*;

    let parsed = t.gitnu_parse("", ["status"]);
    assert!(parsed.is_err());
    assert_eq!(parsed.as_ref().err(), Some(&Error::NotGitRepository));
    assert_eq!(t.sh("", "ls -lA").stdout.trim(), "total 0");
});

// Determined in ../git_cmd.rs
// where it's specified if a command should be skipped because it might
// be part of a flag value
test!(
    skip_flags,
    |t| {
        t.sh("", "git init -b main");
        t.sh("", "touch A B C");
        let _ = t.gitnu("", ["status"]);
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
        t.sh("", "git init -b main");
        t.sh("", "touch A && git add A && git commit -m x");
        t.sh("", "mv A B && git add --all");
        t.sh("", "git nu status");
        let _ = t.gitnu("", ["status"]);
    },
    ["add", "1"],
    ["add", "B"]
);

// Ensure that `gitnu` exit codes match those of `git`. This means
// that error handling bubbles up properly.
test!(exit_codes, |t| {
    macro_rules! assert_code {
        ($cmd:expr, $code:expr) => {
            assert_eq!(t.sh("", $cmd).exit_code, Some($code));
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
    t.sh("", "git init -b main");

    assert_code!("git status", 0);
    assert_code!("git nu status", 0);
});

// Run `gitnu` from a different repository using the `-C` flag.
test!(different_workspace, |t| {
    t.sh("", "mkdir one two");
    t.sh("one", "git init -b one");
    t.sh("two", "git init -b two");
    t.sh("one", "touch gold silver");
    t.sh("two", "git -C ../one nu status");
    t.sh("two", "git -C ../one nu add 1");

    let status = t.sh("two", "git -C ../one nu status");
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
    t.sh("", "git init -b main");
    t.sh("", "touch A && git add A && git commit -m 'first'");
    t.sh("", "git config --global alias.teststatus status");
    let status = t.sh("", "git teststatus");
    t.sh("", "git config --global --unset alias.teststatus");
    assert_eq!(
        status.stdout,
        "\
On branch main
nothing to commit, working tree clean\n"
    );
});

macro_rules! status_test {
    ($name:ident, $setup:expr, $inout:expr, $stdout:expr) => {
        test!($name, |t| {
            let (input, expected) = $inout;

            let setup: Box<dyn Fn(&Test) -> ()> = Box::new($setup);
            setup(t);

            // insert the actual sha where there is "{GIT_SHA}"
            let stdout = if $stdout.contains("{GIT_SHA}") {
                let sha = t.sh("", "git rev-parse --short HEAD");
                $stdout.replace("{GIT_SHA}", sha.stdout.trim())
            } else {
                $stdout.to_string()
            };

            assert_eq!(t.sh("", "git nu status").stdout, stdout);

            // This is necessary because tmp dirs in macOS messes up depending
            // on if you run from shell or in-mem.
            t.gitnu("", ["status"]).unwrap();

            let received = t.gitnu_parse("", input).unwrap();

            assert_eq!(received, expected);
        });
    };
    ($name:ident, $setup:expr, $status:expr) => {
        status_test!($name, $setup, (EM, EM), $status);
    };
}

status_test!(
    git_add_untracked,
    |t| {
        t.sh("", "git init -b main");
        t.sh("", "touch A");
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
        t.sh("", "git init -b main");
        t.sh("", "touch A");
        t.sh("", "git add A && git commit -m x");
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
        t.sh("", "git init -b main");
        t.sh("", "touch A");
        t.sh("", "git add A && git commit -m x");
        t.sh("", "rm A");
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
        t.sh("", "git init -b main");
        t.sh("", "touch A && git add A && git commit -m x");

        // the conflict file
        let basename = "conflict_file";
        let filepath = t.dir.join(basename);

        // left branch
        t.sh("", "git branch -m LEFT");
        fs::write(&filepath, b"LEFT").unwrap();
        t.sh("", format!("git add {basename}"));
        t.sh("", "git commit -m x");

        // right branch
        t.sh("", "git checkout -b RIGHT");
        t.sh("", "git reset --hard HEAD~1");
        fs::write(&filepath, b"RIGHT").unwrap();
        t.sh("", format!("git add {basename}"));
        t.sh("", "git commit -m x");

        // merge and create the conflict
        t.sh("", "git merge LEFT");
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
        t.sh("", "git init -b main");
        for f in "A B C D E F G H I".split(' ') {
            let _ = fs::write(t.dir.join(f), format!("contents::{f}"));
        }
        t.sh("", "git add B C D E G H I");
        t.sh("", "git commit -m pre");
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
        t.sh("", "git add A B C D E _E");
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
        t.sh("", "git init -b main");
        t.sh("", "touch base");
        t.sh("", "git add --all");
        t.sh("", "git commit -m 'base commit'");

        // left branch
        t.sh("", "git branch -m LEFT");
        fs::write(t.dir.join("conflict_file"), b"left").unwrap();
        t.sh("", "git add conflict_file");
        t.sh("", "git commit -m 'left commit'");

        // right branch
        t.sh("", "git checkout -b RIGHT");
        t.sh("", "git reset --hard HEAD~1");
        fs::write(t.dir.join("conflict_file"), b"right").unwrap();
        t.sh("", "git add conflict_file");
        t.sh("", "git commit -m 'right commit'");

        // merge
        t.sh("", "git merge LEFT");
        t.sh("", "touch fresh");
        t.sh("", "git add fresh");
        t.sh("", "git nu status");
        t.sh("", "git nu add 2");
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
        t.sh("", "git init -b main");
        t.sh("", "touch A && git add A && git commit -m 'A'");
        t.sh("", "touch B && git add B && git commit -m 'B'");
        t.sh("", "git checkout HEAD~1");
        t.sh("", "touch gold silver");
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
        t.sh("", "git init -b main");

        t.sh("", {
            let mut args = "touch".to_string();
            (1..25).for_each(|i| args += &format!(" f{i:0>2}"));
            args
        });
        t.sh("", "git nu status");
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

test!(
    max_cache_add_by_number,
    |t| {
        t.sh("", "git init -b main");
        t.sh("", "touch A0 A1 A2 A3 A4 A5 A6 A7 A8 A9 B0 B1 B2 B3 B4 B5 B6 B7 B8 B9 C0 C1 C2");
        let _ = t.gitnu("", ["status"]);
    },
    ["add", "17-20"],
    ["add", "B6", "B7", "B8", "B9"]
);
