use std::env;
use std::fs::File;
use std::path::PathBuf;

pub const TEST_DIR: &str = "gitnu-tests";

pub struct Test {
    pub dir: PathBuf,
}

#[derive(Debug)]
pub struct Output {
    pub stdout: String,
    pub exit_code: Option<i32>,
}

// Get the path to the debug binary
pub fn bin_path() -> String {
    let mut p = env::current_exe().unwrap();
    p.pop();
    p.pop();
    p.push(format!("git-nu{}", env::consts::EXE_SUFFIX));
    p.to_string_lossy().to_string()
}

// Writes to a file by its relative path from test.dir.
pub fn write(t: &Test, file: &str, contents: &str) {
    if let Ok(mut f) = File::create(t.dir.join(file)) {
        use std::io::prelude::Write;
        f.write_all(contents.as_bytes()).ok();
    }
}

/// Runs the test in an isolated directory.
#[macro_export]
macro_rules! test {
    ($name:ident, $fun:expr) => {
        #[test]
        fn $name() {
            #[allow(unused)]
            use crate::parse as gitnu_parse;
            #[allow(unused)]
            use std::{env, fs, path::PathBuf, process::Command};

            fn f() {}
            fn type_name_of<'a, T>(_: T) -> &'a str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            // Get a temporary test directory from a name.
            // This directory will be located at /<tmp>/<TEST_DIR>/<name>
            // where <tmp> is decided by env::temp_dir(), TEST_DIR is a const,
            // and <name> is this function's parameter.
            let test_dir = std::env::temp_dir().join(TEST_DIR).join(&name);
            if test_dir.exists() {
                fs::remove_dir_all(&test_dir).ok();
            }
            fs::create_dir_all(&test_dir).unwrap();

            // Sets the $PATH environment variable such that the debug version of
            // `git-nu` is front-and-center.
            let path = env::var("PATH").unwrap();
            let mut bin = PathBuf::from(bin_path());
            bin.pop();
            env::set_var("PATH", format!("{}:{path}", bin.to_string_lossy()));

            // run the test
            $fun(&Test { dir: test_dir });
        }
    };
}

/// Quickly mock up a gitnu app instance with an optional cwd.
#[macro_export]
macro_rules! gitnu {
    ($t:expr, status) => {{
        gitnu!($t, ["status"]).run().ok()
    }};
    ($t:expr, $args:expr) => {{
        gitnu!($t, "", $args)
    }};
    ($t:expr, $current_dir:expr, $args:expr) => {{
        let git = std::iter::once("git");
        let args = git.chain($args).map(|v| v.to_string());
        gitnu_parse((&$t.dir).join($current_dir), args)
    }};
}

// Run a shell command and extract its stdout and exit code
#[macro_export]
macro_rules! sh {
    ($t:expr, $cmd:expr) => {
        sh!($t, "", $cmd)
    };
    ($t:expr, $cwd:expr, $cmd:expr) => {
        Command::new("sh")
            .current_dir(&$t.dir.join($cwd))
            .arg("-c")
            .arg($cmd.replace("git", "git -c advice.statusHints=false"))
            .output()
            .map(|v| {
                let stdout = String::from_utf8_lossy(&v.stdout).to_string();
                let stderr = String::from_utf8_lossy(&v.stderr).to_string();
                let dir = &$t.dir.join($cwd);
                let dir = dir.to_string_lossy();
                println!("[{}] {}", &dir[dir.len() - 30..], $cmd);
                if !stdout.is_empty() {
                    println!("[stdout]\n{}", stdout);
                }
                if !stderr.is_empty() {
                    println!("[stderr]\n({})", stderr);
                }
                Output { stdout, exit_code: v.status.code() }
            })
            .unwrap()
    };
}

/// Makes an assertion of the list of command line arguments that
/// `gitnu` will pass back to the terminal after processing.
#[macro_export]
macro_rules! assert_args {
    ($received_app:expr, $expected:expr) => {{
        // extract arguments into a list
        let cmd = $received_app.cmd();
        let args = cmd.get_args();
        let mut all_args: Vec<String> =
            Vec::with_capacity(cmd.get_args().len() + 1);
        all_args.push(cmd.get_program().to_string_lossy().to_string());
        all_args.extend(args.map(|v| v.to_string_lossy().to_string()));

        // remove the sub-sequence ["-c", "color.ui=always"]
        let remove_index = (0..all_args.len() - 1).find(|i| {
            all_args[*i].eq("-c") && all_args[i + 1].eq("color.ui=always")
        });
        if let Some(i) = remove_index {
            all_args.remove(i);
            all_args.remove(i);
        }

        let expected: Vec<String> =
            $expected.iter().map(|v| v.to_string()).collect();
        assert_eq!(all_args, expected);
    }};
}

#[macro_export]
macro_rules! assert_eq_pretty {
    ($received:expr, $expected:expr) => {
        if $received != $expected {
            panic!(
                "Printed outputs differ!\n
received ↓
<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
{received}
============================================================
{expected}
>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
expected ↑\n",
                received = $received,
                expected = $expected
            );
        }
    };
}
