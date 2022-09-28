use crate::files::Cache;
use crate::opts::Opts;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind::Other};
use std::path::PathBuf;
use std::process::ExitStatus;

pub trait CacheActions {
    fn write_cache(&self, content: String) -> Option<()>;
    fn read_cache(&self) -> Option<Cache>;
}
pub trait RunAction {
    fn run(&self, args: Vec<PathBuf>) -> Result<ExitStatus, Error>;
}

impl CacheActions for Opts {
    fn write_cache(&self, content: String) -> Option<()> {
        std::fs::write(self.cache_file()?, content).ok()
    }
    fn read_cache(&self) -> Option<Cache> {
        let file = std::fs::File::open(self.cache_file()?).ok()?;
        let mut cache = Cache::new();
        let git_root = self.git_root.as_ref()?;
        let add = |v| cache.push(Some(git_root.join(v)));
        BufReader::new(file).lines().filter_map(|v| v.ok()).for_each(add);
        Some(cache)
    }
}

impl RunAction for Opts {
    fn run(&self, args: Vec<PathBuf>) -> Result<ExitStatus, Error> {
        let err = Error::new(Other, "Unable to run");
        self.cmd().ok_or(err)?.args(args).spawn()?.wait()
    }
}
