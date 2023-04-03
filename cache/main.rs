use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

fn lines<R: Read>(src: R) -> impl Iterator<Item = String> {
    BufReader::new(src).lines().filter_map(|v| v.ok())
}

fn test_filepath() -> PathBuf {
    let mut p = std::env::current_exe().unwrap();
    for _ in 0..3 {
        p.pop();
    }
    p.join("cache/test-cache.txt")
}

fn test_file() -> File {
    File::open(test_filepath()).unwrap()
}

fn main() {
    println!("-> {:?}", test_file());
    println!("cache!");
}
