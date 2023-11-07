use crate::prelude::*;
use crate::App;

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

pub(crate) const TEST_DIR: &str = "gitnu-tests";

pub(crate) struct Test {
    pub dir: PathBuf,
}

#[derive(Debug)]
pub(crate) struct Output {
    pub stdout: String,
    pub exit_code: Option<i32>,
}

/// Get the path to the debug binary
pub(crate) fn bin_dir() -> String {
    let mut p = env::current_exe().unwrap();
    (p.pop(), p.pop());
    p.to_string_lossy().trim().to_string()
}

// Writes to a file by its relative path from test.dir.
pub(crate) fn write(t: &Test, file: &str, contents: &str) {
    if let Ok(mut f) = File::create(t.dir.join(file)) {
        f.write_all(contents.as_bytes()).ok();
    }
}

pub(crate) fn git_shell<S: AsRef<str>>(cwd: &PathBuf, cmd: S) -> Output {
    let cmd = match cmd.as_ref().starts_with("git") {
        false => cmd.as_ref().to_string(),
        _ => cmd.as_ref().replace("git", "git -c advice.statusHints=false"),
    };
    Command::new("sh")
        .current_dir(&cwd)
        .arg("-c")
        .arg(&cmd)
        .output()
        .map(|v| {
            let stdout = String::from_utf8_lossy(&v.stdout).to_string();
            let stderr = String::from_utf8_lossy(&v.stderr).to_string();
            println!("╭───────────────────── RUN SH ─────────────────────╮");
            println!("> [ \x1b[0;32m{}\x1b[0m ]", cmd);
            println!("relative dir: \x1b[0;32m{}\x1b[0m", cwd.display());
            println!("::: STDOUT :::\n{}", stdout);
            println!("::: STDERR :::\n{}", stderr);
            println!("╰──────────────────────────────────────────────────╯");
            Output { stdout, exit_code: v.status.code() }
        })
        .unwrap()
}

/// Gets an environment variable with a maximum of 100 retries.
pub(crate) fn env_var(name: &str) -> String {
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

pub(crate) fn mock_app<S, P>(cwd: P, args: &[S]) -> Result<App>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    use crate::cli_init_app;
    let mut app = cli_init_app(cwd.as_ref().to_path_buf())?;
    let args = {
        let mut t = vec!["git".to_string()];
        t.extend(args.iter().map(|v| v.as_ref().to_string()));
        t
    };
    // forcefully run the test binary in the test directory
    app.final_cmd.current_dir(&cwd);
    app.parse(&string_vec(args));
    Ok(app)
}

/// 1. Clear and re-create the test directory
/// 2. Set the $PATH to ensure that the debug binary is front-and-center.
pub(crate) fn prep_test(name: &str) -> PathBuf {
    let test_dir = env::temp_dir().join(TEST_DIR).join(&name);
    if test_dir.exists() {
        fs::remove_dir_all(&test_dir).ok();
    }
    fs::create_dir_all(&test_dir).unwrap();

    let path = env_var("PATH");
    env::set_var("PATH", format!("{}:{path}", bin_dir()));

    test_dir
}

/// Runs the test in an isolated directory.
macro_rules! test {
    ($name:ident, $fun:expr) => {
        #[test]
        fn $name() {
            use $crate::tests::macros::*;
            fn f() {}
            fn type_name_of<'a, T>(_: T) -> &'a str {
                std::any::type_name::<T>()
            }
            let test_dir = prep_test(type_name_of(f));
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
            use $crate::prelude::*;
            let setup: Box<dyn Fn(&Test) -> ()> = Box::new($setup);
            setup(t);
            let parsed = gitnu!(t, $relative_dir, $input_args).unwrap();
            let received_args = parsed.final_cmd.real_args();
            assert_eq!(received_args, $output_args);
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
        let cwd = $t.dir.join($relative_dir);
        mock_app(cwd, &$crate::prelude::string_vec($args))
    }};
}

// Run a shell command and extract its stdout and exit code
macro_rules! sh {
    ($t:expr, $cmd:expr) => {
        sh!($t, "", $cmd)
    };
    ($t:expr, $cwd:expr, $cmd:expr) => {
        git_shell(&$t.dir.join($cwd), $cmd)
    };
}
