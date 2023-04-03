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

fn pure_git_status() {}

fn main() {
    println!("start benching cache!");
    let test_file = here().join("cache/texts/262144.txt");
    let avg = bench(|| read_cache(&test_file), 10);
    println!("avg time: {:?}", avg);
    println!("end of cache bench!");
}
