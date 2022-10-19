use crate::opts::Opts;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub type Cache = Vec<Option<PathBuf>>;

fn is_short_flag(s: &str, skip: &mut bool) -> bool {
    let mut c = s.chars();
    *skip = c.next() == Some('-') && c.next() != Some('-');
    *skip
}

fn get_range(arg: &str) -> Option<[usize; 2]> {
    arg.parse().map(|v| Some([v, v])).unwrap_or_else(|_| {
        let (a, b) = arg.split_once("-")?;
        let a = a.parse().ok()?;
        let b = b.parse().unwrap_or(a);
        Some(if a < b { [a, b] } else { [b, a] })
    })
}

pub fn load(args: Vec<String>, opts: &Opts) -> Vec<PathBuf> {
    let mut res: Vec<PathBuf> = Vec::new();
    let cache = || -> Option<Cache> {
        let f = BufReader::new(File::open(opts.cache_file()?).ok()?).lines();
        Some(f.filter_map(|v| v.ok()).map(PathBuf::from).map(Some).collect())
    };
    let mut cache = cache().unwrap_or(Cache::new());
    let skip = &mut false;
    args.iter().for_each(|a| match *skip || is_short_flag(a, skip) {
        true => res.push(PathBuf::from(a)),
        false => match get_range(a) {
            None => res.push(PathBuf::from(a)),
            Some([s, e]) => (s..e + 1)
                .filter_map(|n| std::mem::take(cache.get_mut(n - 1)?))
                .for_each(|v| res.push(v)),
        },
    });
    return res;
}
