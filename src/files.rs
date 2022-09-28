use crate::opts::{Cache, Opts};
use std::mem;
use std::path::PathBuf;

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
            Ok(n) => mem::take(self.cache.get_mut(n - 1)?),
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
    use crate::opts::CacheActions;
    let cache = opts.read_cache().unwrap_or(Vec::new());

    // make a wrapper to safely apply to args
    Files::new(cache).apply(args)
}

#[test]
/// after getting once, getting the same index should return None
fn test_get() -> Result<(), ()> {
    // mock args ["1", "1"]
    let mut args: Vec<String> = Vec::new();
    args.push(String::from("1"));
    args.push(String::from("1"));
    // mock cache ["/one", "/two"]
    let mut cache: Cache = Vec::new();
    cache.push(Some(PathBuf::from("/one")));
    cache.push(Some(PathBuf::from("/two")));
    let mut files = Files::new(cache);
    let output = files.apply(args);
    assert_eq!(output.len(), 1);
    assert_eq!(output[0], PathBuf::from("/one"));
    assert_eq!(files.get(&String::from("1")), None);
    Ok(())
}
