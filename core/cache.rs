use crate::{git, App};
use std::io::{BufRead, BufReader};
use std::{fs::File, path::PathBuf};

pub trait Cache {
    /// Obtain a handler to the cache file.
    fn cache_file(&self, create: bool) -> Option<File>;

    /// Adds a range of files by index as arguments to the `Command` that will
    /// eventually be run.
    ///
    /// Loads files indexed [start, end] (inclusive)
    fn load_range(&mut self, start: usize, end: usize);

    /// Lazily loads cache file into buffer without actually reading any lines
    /// yet. Should only be called when confirmed App's subcommand is of the
    /// `Number` variant.
    fn load_cache_buffer(&mut self);
}

impl App {
    /// use the path obtained from `git rev-parse --git-dir` to store the cache.
    /// this is usually the .git folder of regular repositories, and somewhere
    /// deeper for worktrees.
    fn cache_path(&self) -> Option<PathBuf> {
        // git.stdout returns the git-dir relative to cwd,
        // so prepend it with current dir
        git::git_dir(self.cmd.get_args().take(self.argc))
            .map(|v| self.cwd.join(v).join("gitnu.txt"))
    }

    fn read_until(&mut self, n: usize) {
        let len = self.cache.len();
        if n < len || self.buffer.is_none() {
            return;
        }
        let buffer = self.buffer.as_mut().unwrap().take(n + 1 - len);
        self.cache.extend(
            buffer.enumerate().map(|(i, v)| v.unwrap_or((len + i).to_string())),
        );
    }
}

impl Cache for App {
    fn cache_file(&self, create: bool) -> Option<File> {
        self.cache_path().and_then(|v| match create {
            true => File::create(v).ok(),
            false => File::open(v).ok(),
        })
    }

    fn load_range(&mut self, start: usize, end: usize) {
        self.read_until(end);
        (start..end + 1).for_each(|n| match self.cache.get(n) {
            Some(pathspec) => {
                self.cmd.arg(self.cache_cwd.join(pathspec));
            }
            None => {
                self.cmd.arg(n.to_string());
            }
        });
    }

    fn load_cache_buffer(&mut self) {
        self.cache = vec!["0".to_string()];
        if let Some(file) = self.cache_file(false) {
            let mut buffer = BufReader::new(file).lines();
            self.cache_cwd = PathBuf::from(buffer.next().unwrap().unwrap());
            self.buffer = Some(buffer);
        }
    }
}
