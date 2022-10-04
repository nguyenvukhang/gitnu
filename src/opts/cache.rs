use crate::opts::{Cache, CacheOps, Opts};
use std::fs::File;
use std::io::{BufRead, BufReader};

impl CacheOps for Opts {
    fn read_cache(&self) -> Option<Cache> {
        let mut cache = Cache::new();
        let add = |v| cache.push(Some(std::path::PathBuf::from(v)));
        BufReader::new(File::open(self.cache_file()?).ok()?)
            .lines()
            .filter_map(|v| v.ok())
            .for_each(add);
        Some(cache)
    }
}
