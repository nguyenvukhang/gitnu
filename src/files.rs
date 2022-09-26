// gitnu add 2-4
// gitnu reset 7
use crate::opts::Opts;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
struct Files {
    data: HashMap<u16, PathBuf>,
    count: u16,
}

impl Files {
    pub fn new(data: HashMap<u16, PathBuf>) -> Files {
        Files { data, count: 0 }
    }
    fn get(&mut self, query: &OsString) -> Option<PathBuf> {
        let index: u16 = match query.to_str().unwrap_or("").parse() {
            Ok(0) => return None,
            Ok(v) => v,
            Err(_) => return None,
        };
        self.data.remove(&index)
    }
    pub fn apply(&mut self, args: &mut Vec<OsString>) {
        fn is_flag(flag: &OsString) -> bool {
            flag.to_str().unwrap_or("").starts_with("-")
                && flag.to_ascii_lowercase().eq(flag)
        }
        let mut it = args.iter_mut();
        while let Some(arg) = it.next() {
            // don't parse things like the 15 in `gitnu log -n 15`
            // once a flag is seen, skip both the flag and the next arg
            if is_flag(arg) {
                it.next();
                continue;
            }
            if let Some(res) = self.get(arg) {
                *arg = res.into_os_string();
                self.count += 1;
            }
        }
    }
    pub fn count(&self) -> u16 {
        return self.count;
    }
}

/// replace numbers with filenames
/// mutates the vector passed in, since the result has the same length
pub fn load(args: &mut Vec<OsString>, opts: &Opts) {
    // read cache
    let cache = opts.read_cache().unwrap_or(HashMap::new());

    // make a wrapper to safely apply to args
    let mut files = Files::new(cache);
    files.apply(args);

    log::info!("made {} replacements", files.count());
    log::info!("files::load {:?}", args);
}
