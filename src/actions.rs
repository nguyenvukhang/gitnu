use crate::opts::Opts;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind::NotFound};
use std::path::PathBuf;
use std::process::ExitStatus;

pub type FileIndex = std::collections::HashMap<usize, PathBuf>;

pub trait CacheActions {
    fn write_cache(&self, content: String) -> Result<(), Error>;
    fn read_cache(&self) -> Result<FileIndex, Error>;
}
pub trait RunAction {
    fn run(&self, args: Vec<PathBuf>) -> Result<ExitStatus, Error>;
}

impl CacheActions for Opts {
    fn write_cache(&self, content: String) -> Result<(), Error> {
        return std::fs::write(self.cache_file()?, content);
    }
    fn read_cache(&self) -> Result<FileIndex, Error> {
        let file = std::fs::File::open(self.cache_file()?)?;
        let mut data: FileIndex = FileIndex::new();
        let mut count = 1;
        let git_root = &self
            .git_root
            .as_ref()
            .ok_or(Error::new(NotFound, "git workspace root not found"))?;
        let insert = |v| {
            data.insert(count, git_root.join(v));
            count += 1;
        };
        BufReader::new(file).lines().filter_map(|v| v.ok()).for_each(insert);
        Ok(data)
    }
}

impl RunAction for Opts {
    fn run(&self, args: Vec<PathBuf>) -> Result<ExitStatus, Error> {
        self.cmd()?.args(args).spawn()?.wait()
    }
}
