// gitnu add 2-4
// gitnu reset 7
use crate::opts::Opts;
use std::collections::HashMap;

#[derive(Debug)]
struct Files {
    data2: HashMap<u16, String>,
    count: u16,
}

impl Files {
    pub fn new(data2: HashMap<u16, String>) -> Files {
        Files { data2, count: 0 }
    }
    fn get(&mut self, query: &str) -> Option<String> {
        let index: u16 = match query.parse() {
            Ok(0) => return None,
            Ok(v) => v,
            Err(_) => return None,
        };
        self.data2.remove(&index)
    }
    pub fn apply(&mut self, args: &mut Vec<String>) {
        fn is_flag(flag: &str) -> bool {
            flag.starts_with("-") && flag.to_ascii_lowercase().eq(flag)
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
                *arg = res;
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
pub fn load(args: &mut Vec<String>, opts: &Opts) {
    // read cache
    let cache = opts.read_cache2().unwrap_or(HashMap::new());

    // make a wrapper to safely apply to args
    let mut files = Files::new(cache);
    files.apply(args);

    log::info!("made {} replacements", files.count());
    log::info!("files::load {:?}", args);
}
