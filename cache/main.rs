use std::{
    fmt::Debug,
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
};

fn lines<R: Read>(src: R) -> impl Iterator<Item = String> {
    BufReader::new(src).lines().filter_map(|v| v.ok())
}

fn here() -> PathBuf {
    let mut p = std::env::current_exe().unwrap();
    for _ in 0..3 {
        p.pop();
    }
    p
}

fn bench<T: Debug, F: Fn() -> T>(f: F, times: usize) -> Duration {
    let mut t = Duration::ZERO;
    let start = Instant::now();
    writeln!(io::stdout(), "warming up...").ok();
    while Instant::elapsed(&start) < Duration::from_millis(2000) {
        f();
    }
    writeln!(io::stdout(), "running...").ok();
    for i in 0..times {
        writeln!(io::stdout(), "#{i}").ok();
        let start = Instant::now();
        f();
        t += Instant::elapsed(&start);
    }
    writeln!(io::stdout(), "done!").ok();
    t / times as u32
}

/// the main function to bench
fn read_cache(file: &Path) -> Vec<String> {
    let f = File::open(file).unwrap();
    let mut cmd = Command::new("git");
    lines(f).for_each(|v| {
        cmd.arg(v);
    });
    Vec::from_iter(cmd.get_args().take(4).map(|v| {
        let mut v = v.to_string_lossy().to_string();
        v.truncate(10);
        v
    }))
}

macro_rules! sh {
    ($cmd:expr) => {
        sh!($t, "", $cmd)
    };
    ($cwd:expr, $cmd:expr) => {
        Command::new("sh")
            .current_dir($cwd)
            .arg("-c")
            .arg($cmd)
            .output()
            .unwrap()
    };
}

fn pure_git_status(cwd: &Path) {
    sh!(cwd, "git init");
    let x = sh!(cwd, "git status");
    let count = x.stdout.into_iter().filter(|v| *v == b'\n').count();
    assert!(count > 65536);
    sh!(cwd, "rm -rf .git");
}

fn main() {
    println!("start benching cache!");

    let limit = 65536;

    // raw read speeds
    let test_file = here().join(format!("cache/texts/{limit}.txt"));
    let avg = bench(|| read_cache(&test_file), 10);
    println!("avg raw read speed: {:?}", avg);

    // git status speeds
    let tmp_dir = std::env::temp_dir().join("gitnu-cache-tests");
    std::fs::create_dir_all(&tmp_dir).unwrap();
    for i in 0..4 {
        let mut touch = Command::new("touch");
        touch.current_dir(&tmp_dir);
        for j in 0..limit / 4 {
            touch.arg((limit / 4 * i + j).to_string() + ".txt");
        }
        touch.output().unwrap();
    }
    Command::new("git").arg("init").current_dir(&tmp_dir).output().unwrap();
    let avg = bench(|| pure_git_status(&tmp_dir), 5);
    println!("avg pure git status: {:?}", avg);

    println!("deleting {:?}", tmp_dir);
    sh!(&tmp_dir, format!("rm -rf {}", tmp_dir.to_string_lossy()));

    println!("end of cache bench!");
}
