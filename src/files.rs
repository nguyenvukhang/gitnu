use crate::opts::Opts;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub type Cache = Vec<Option<PathBuf>>;

pub trait FileActions {
    fn get(&mut self, query: &String) -> Option<PathBuf>;
    fn apply(&mut self, args: Vec<String>) -> Vec<PathBuf>;
}

impl FileActions for Cache {
    fn get(&mut self, query: &String) -> Option<PathBuf> {
        match query.parse::<usize>() {
            Err(_) => Some(query.into()), // not even an integer
            Ok(n) => std::mem::take(self.get_mut(n - 1)?),
        }
    }

    fn apply(&mut self, args: Vec<String>) -> Vec<PathBuf> {
        let (mut skip, mut is_flag) = (false, false);
        let apply = |a: &String| {
            is_flag = a.starts_with('-') && a.to_ascii_lowercase().eq(a);
            if skip || is_flag {
                skip = is_flag;
                return Some(PathBuf::from(a));
            }
            self.get(&a)
        };
        args.iter().filter_map(apply).collect()
    }
}

pub fn load(args: Vec<String>, opts: &Opts) -> Vec<PathBuf> {
    let cache = || -> Option<Cache> {
        let mut cache = Cache::new();
        let br = BufReader::new(File::open(opts.cache_file()?).ok()?);
        let add = |v: String| cache.push(Some(v.into()));
        br.lines().filter_map(|v| v.ok()).for_each(add);
        Some(cache)
    };
    cache().unwrap_or(Cache::new()).apply(args)
}
