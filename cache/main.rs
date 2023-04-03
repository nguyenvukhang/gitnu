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
    for _ in 0..times {
        let start = Instant::now();
        let x = f();
        println!("{:?}", x);
        t += Instant::elapsed(&start);
    }
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
    println!("--------- {:?}", cwd);
    sh!(cwd, "git init");
    let x = sh!(cwd, "git status");
    sh!(cwd, "rm -rf .git");
    println!("{x:?}")
}

fn main() {
    println!("start benching cache!");

    let limit = 65536;

    // raw read speeds
    // let test_file = here().join(format!("cache/texts/{limit}.txt"));
    // let avg = bench(|| read_cache(&test_file), 10);
    // println!("avg raw read speed: {:?}", avg);

    // git status speeds
    let tmp_dir = std::env::temp_dir().join("gitnu-cache-tests");
    std::fs::create_dir_all(&tmp_dir).unwrap();
    for i in 0..limit / 8 {
        let p = tmp_dir.join(i.to_string() + ".txt");
        println!("{i}");
        writeln!(File::create(p).unwrap(), "_").unwrap();
    }
    Command::new("git").arg("init").current_dir(&tmp_dir).output().unwrap();
    // let avg = bench(|| pure_git_status(&tmp_dir), 1);
    pure_git_status(&tmp_dir);
    // println!("avg pure git status: {:?}", avg);

    println!("deleting {:?}", tmp_dir);
    std::fs::remove_dir_all(&tmp_dir).unwrap();

    println!("end of cache bench!");
}
