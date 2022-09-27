use crate::actions::FileIndex;
use crate::opts::Opts;
use std::collections::HashMap;
use std::path::PathBuf;

struct Files {
    /// stored as PathBuf because this needs to be joined
    /// with the path to git workspace root
    data: FileIndex,

    /// highest number that has a file indexed
    max: usize,
}

impl Files {
    pub fn new(data: FileIndex) -> Files {
        assert_eq!(data.get(&0), None);
        Files { max: data.len() + 1, data }
    }
    fn get(&mut self, query: &String) -> Option<PathBuf> {
        let index = match query.parse() {
            Err(_) => return Some(query.into()), // not even an integer
            Ok(i) if i > self.max => return None, // out of range
            Ok(v) => v,
        };
        self.data.remove(&index)
    }
    pub fn apply(&mut self, args: Vec<String>) -> Vec<PathBuf> {
        let mut skip = false;
        let mut flag = false;
        let is_flag =
            |f: &str| f.starts_with("-") && f.to_ascii_lowercase().eq(f);
        let apply = |a: &String| -> Option<PathBuf> {
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
    let cache = opts.read_cache().unwrap_or(HashMap::new());

    // make a wrapper to safely apply to args
    Files::new(cache).apply(args)
}
