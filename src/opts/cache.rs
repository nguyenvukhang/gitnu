use crate::opts::{Cache, CacheOps, Opts};
use std::fs::File;
use std::io::{BufRead, BufReader};

impl CacheOps for Opts {
    fn write_cache(&self, content: String) -> Option<()> {
        std::fs::write(self.cache_file()?, content).ok()
    }
    fn read_cache(&self) -> Option<Cache> {
        let mut cache = Cache::new();
        let git_root = self.git_root.as_ref()?;
        let add = |v| cache.push(Some(git_root.join(v)));
        BufReader::new(File::open(self.cache_file()?).ok()?)
            .lines()
            .filter_map(|v| v.ok())
            .for_each(add);
        Some(cache)
    }
}
