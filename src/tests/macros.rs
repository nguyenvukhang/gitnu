use std::env;
use std::fs::File;
use std::io::Write;
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
pub fn bin_dir() -> String {
    let mut p = env::current_exe().unwrap();
    p.pop();
    p.pop();
    p.to_string_lossy().trim().to_string()
}

// Writes to a file by its relative path from test.dir.
pub fn write(t: &Test, file: &str, contents: &str) {
    if let Ok(mut f) = File::create(t.dir.join(file)) {
        f.write_all(contents.as_bytes()).ok();
    }
}

pub fn env_var(name: &str) -> String {
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

/// Runs the test in an isolated directory.
macro_rules! test {
    ($name:ident, $fun:expr) => {
        #[test]
        fn $name() {
            #[allow(unused_imports)]
            use std::{env, fs, path::PathBuf, process::Command};
            use $crate::tests::macros::*;

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
            let path = env_var("PATH");
            env::set_var("PATH", format!("{}:{path}", bin_dir()));

            // run the test
            let fun: Box<dyn Fn(&Test) -> ()> = Box::new($fun);
            fun(&Test { dir: test_dir });
        }
    };
    ($name:ident, $setup:expr, $input_args:expr, $output_args:expr) => {
        test!($name, $setup, "", $input_args, $output_args);
    };
    ($name:ident, $setup:expr, $relative_dir:expr, $input_args:expr, $output_args:expr) => {
        test!($name, |t| {
            let setup: Box<dyn Fn(&Test) -> ()> = Box::new($setup);
            setup(t);

            let parsed = gitnu!(t, $relative_dir, $input_args).unwrap();
            let received_args = parsed.final_command.get_args();

            assert_eq!(received_args, $output_args);
            // assert_args!(gitnu!(t, $input_args), $output_args);
        });
    };
}

/// Quickly mock up a gitnu app instance with an optional cwd.
macro_rules! gitnu {
    ($t:expr, status) => {{
        gitnu!($t, ["status"]).and_then(|v| v.run())
    }};
    ($t:expr, $args:expr) => {{
        gitnu!($t, "", $args)
    }};
    ($t:expr, $relative_dir:expr, status) => {{
        gitnu!($t, $relative_dir, ["status"]).and_then(|v| v.run())
    }};
    // Returns a parsed app, but not ran.
    ($t:expr, $relative_dir:expr, $args:expr) => {{
        let mut args = Vec::with_capacity($args.len() + 1);
        args.push("git");
        args.extend($args);

        let cwd = $t.dir.join($relative_dir);
        let git_dir = $crate::git::relative_dir(&cwd);

        git_dir.map(|git_dir| {
            let cache = $crate::Cache::new(&git_dir, &cwd);
            let app = $crate::AppBuilder::new()
                .current_dir(&cwd)
                .cache(cache)
                .git_dir(git_dir)
                .build();
            app.parse(args.into_iter().map(String::from))
        })
    }};
}

// Run a shell command and extract its stdout and exit code
macro_rules! sh {
    ($t:expr, $cmd:expr) => {
        sh!($t, "", $cmd)
    };
    ($t:expr, $cwd:expr, $cmd:expr) => {
        Command::new("sh")
            .current_dir(&$t.dir.join($cwd))
            .arg("-c")
            .arg({
                if $cmd.starts_with("git") {
                    $cmd.replace("git", "git -c advice.statusHints=false")
                } else {
                    $cmd.to_string()
                }
            })
            .output()
            .map(|v| {
                let line = "─────────────────────────";
                let stdout = String::from_utf8_lossy(&v.stdout).to_string();
                let stderr = String::from_utf8_lossy(&v.stderr).to_string();
                println!("╭{line} RUN SH {line}╮");
                println!("test dir: {}", $t.dir.to_string_lossy());
                println!("relative dir: \x1b[0;32m{}\x1b[0m", $cwd);
                println!("cmd:          \x1b[0;32m{}\x1b[0m", $cmd);
                if !stdout.is_empty() {
                    println!(" {line} STDOUT {line}\n{}", stdout);
                }
                if !stderr.is_empty() {
                    println!(" {line} STDERR {line}\n{}", stderr);
                }
                println!("╰{line}────────{line}╯");
                Output { stdout, exit_code: v.status.code() }
            })
            .unwrap()
    };
}

/// Makes an assertion of the list of command line arguments that
/// `gitnu` will pass back to the terminal after processing.
macro_rules! assert_args {
    ($received_app:expr, $expected:expr) => {{
        // extract arguments into a list
        let args = $received_app.final_command.get_args();

        let expected: Vec<String> =
            $expected.iter().map(|v| v.to_string()).collect();
        assert_eq!(args, expected);
    }};
    ($test:expr, $received:expr, $expected:expr) => {{
        let app = gitnu!($test, $received);
        let received = app.git.get_string_args();

        let expected: Vec<String> =
            $expected.iter().map(|v| v.to_string()).collect();

        assert_eq!(received, expected);
    }};
}
