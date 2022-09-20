// gitnu add 2-4
// gitnu reset 7
use crate::opts::Opts;

#[derive(Debug)]
struct Files {
    data: Vec<String>,
    count: u16,
}

impl Files {
    pub fn new(data: Vec<String>) -> Files {
        log::info!("Files::new <- {:?}", data);
        Files { data, count: 0 }
    }
    fn get(&self, query: &str) -> Option<String> {
        let index: usize = match query.parse() {
            Ok(0) => return None,
            Ok(v) => v,
            Err(_) => return None,
        };
        match self.data.get(index - 1) {
            None => return None,
            Some(v) => Some(v.to_string()),
        }
    }
    pub fn apply(&mut self, args: &mut Vec<String>) {
        for i in 0..args.len() {
            let res = self.get(&args[i]);
            if res.is_some() {
                args[i] = res.unwrap();
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
    let cache = opts.read_cache().unwrap_or(Vec::new());

    // make a wrapper to safely apply to args
    let mut files = Files::new(cache);
    files.apply(args);

    log::info!("made {} replacements", files.count());
    log::info!("files::load {:?}", args);
}
