use crate::opts::{Cache, Opts};
use std::mem;
use std::path::PathBuf;

trait FileActions {
    /// Gets a PathBuf from the cache without making copies.
    /// This is done by swapping out the vector element with a None
    /// and returning the element itself.
    ///
    /// This means that each element can only be extracted once.
    fn get(&mut self, query: &String) -> Option<PathBuf>;

    /// Applies file cache onto an array of command line arguments,
    /// but skips over flagged values. For example, the '10' in
    /// `git log -n 10` will not be replaced with a filename because
    /// it's used as the value of the -n flag.
    fn apply(&mut self, args: Vec<String>) -> Vec<PathBuf>;
}

impl FileActions for Cache {
    fn get(&mut self, query: &String) -> Option<PathBuf> {
        match query.parse::<usize>() {
            Err(_) => Some(query.into()), // not even an integer
            Ok(n) => mem::take(self.get_mut(n - 1)?),
        }
    }

    fn apply(&mut self, args: Vec<String>) -> Vec<PathBuf> {
        let (mut skip, mut flag) = (false, false);
        let is_flag =
            |f: &str| f.starts_with('-') && f.to_ascii_lowercase().eq(f);
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

/// Replace numbers with filenames.
/// Returns a new PathBuf vector
pub fn load(args: Vec<String>, opts: &Opts) -> Vec<PathBuf> {
    // read cache
    use crate::opts::CacheOps;
    let mut cache = opts.read_cache().unwrap_or(Cache::new());

    // make a wrapper to safely apply to args
    cache.apply(args)
}

#[test]
/// after getting once, getting the same index should return None
fn test_get() {
    let mut cache: Cache =
        ["/one", "/two"].iter().map(|v| Some(PathBuf::from(v))).collect();
    assert_eq!(cache.get(&"1".into()), Some("/one".into()));
    assert_eq!(cache.get(&"1".into()), None);
    assert_eq!(cache.get(&"2".into()), Some("/two".into()));
    assert_eq!(cache.get(&"2".into()), None);
    assert_eq!(cache.get(&"3".into()), None);
}
