use crate::pathdiff;
use crate::prelude::*;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Default)]
pub struct Cache {
    prefix: Option<PathBuf>,
    files: Vec<String>,
}

impl Cache {
    pub fn new<P>(git_dir: &PathBuf, cwd: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::try_read(git_dir, cwd).unwrap_or_default()
    }

    fn try_read<P>(git_dir: &PathBuf, cwd: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut filepath = cwd.as_ref().to_path_buf();
        filepath.push(git_dir);
        filepath.push(CACHE_FILE_NAME);

        let file = File::open(filepath)?;
        let mut lines = BufReader::new(file).lines().filter_map(|v| v.ok());

        let prefix = {
            let first_line = lines.next().ok_or(Error::InvalidCache)?;
            let prefix = PathBuf::from(first_line);
            match pathdiff::diff_paths(prefix, cwd) {
                Some(v) if v.as_os_str().is_empty() => None,
                v => v,
            }
        };

        let mut files = Vec::with_capacity(MAX_CACHE_SIZE);
        files.push(0.to_string());
        files.extend(lines.take(MAX_CACHE_SIZE - 1));

        Ok(Self { prefix, files })
    }

    pub fn load(&self, index: usize, cmd: &mut Command) {
        match (&self.prefix, self.files.get(index)) {
            (Some(prefix), Some(pathspec)) => cmd.arg(prefix.join(pathspec)),
            (None, Some(pathspec)) => cmd.arg(pathspec),
            _ => cmd.arg(index.to_string()),
        };
    }
}
