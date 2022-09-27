use crate::opts::Opts;
use std::path::PathBuf;

pub type Cache = Vec<PathBuf>;

struct Files {
    /// stored as PathBuf because this needs to be joined
    /// with the path to git workspace root
    cache: Cache,
}

impl Files {
    pub fn new(cache: Cache) -> Files {
        Files { cache }
    }
    fn get(&mut self, query: &String) -> Option<PathBuf> {
        match query.parse::<usize>() {
            Err(_) => Some(query.into()), // not even an integer
            Ok(n) => self.cache.get(n - 1).map(|v| v.to_owned()),
        }
    }
    pub fn apply(&mut self, args: Vec<String>) -> Vec<PathBuf> {
        let mut skip = false;
        let mut flag = false;
        let is_flag =
            |f: &str| f.starts_with("-") && f.to_ascii_lowercase().eq(f);
        let apply = |a: &String| {
            flag = is_flag(&a);
            if skip || flag {
                skip = flag;
                return Some(PathBuf::from(a));
            }
            self.get(&a)
        };
        args.iter().filter_map(apply).collect()
    }
}

/// replace numbers with filenames
/// mutates the vector passed in, since the result has the same length
pub fn load(args: Vec<String>, opts: &Opts) -> Vec<PathBuf> {
    // read cache
    use crate::actions::CacheActions;
    let cache = opts.read_cache().unwrap_or(Vec::new());

    // make a wrapper to safely apply to args
    Files::new(cache).apply(args)
}
