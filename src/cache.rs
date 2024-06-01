use crate::prelude::*;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct Cache {
    prefix: Option<PathBuf>,
    files: Vec<String>,
}

impl Cache {
    /// Initialize cache by reading the cache file in `git_dir`.
    pub fn new<P>(git_dir: &PathBuf, cwd: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::try_read(git_dir, cwd).unwrap_or_default()
    }

    /// Try to read the cache file from `git_dir`.
    fn try_read<P>(git_dir: &PathBuf, cwd: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut cache_path = cwd.as_ref().to_path_buf();
        cache_path.push(git_dir);
        cache_path.push(CACHE_FILE_NAME);

        let f = File::open(cache_path)?;
        let mut lines = BufReader::new(f).lines().filter_map(|v| v.ok());

        let prefix = {
            let first_line = lines.next().ok_or(Error::InvalidCache)?;
            let prefix = PathBuf::from(first_line);
            match pathdiff::diff_paths(prefix, cwd) {
                Some(v) if v.as_os_str().is_empty() => None,
                v => v,
            }
        };

        let mut files = Vec::with_capacity(MAX_CACHE_SIZE + 1);
        files.push(0.to_string());
        files.extend(lines.take(MAX_CACHE_SIZE));

        Ok(Self { prefix, files })
    }

    /// Append the `index`-th cached value into an ArgHolder.
    pub fn load<A: ArgHolder>(&self, index: usize, argh: &mut A) {
        match (&self.prefix, self.files.get(index)) {
            (Some(prefix), Some(pathspec)) => {
                argh.add_arg(prefix.join(pathspec))
            }
            (None, Some(pathspec)) => argh.add_arg(pathspec),
            _ => argh.add_arg(index.to_string()),
        };
    }
}
