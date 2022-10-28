#[cfg(test)]
mod tests {
    use crate::test;
    use crate::{parser::parse, Opts};
    use std::path::PathBuf;

    pub fn _parse(args: &[&str], path: &str) -> Opts {
        parse(test::iter([&["gitnu"], args].concat()), PathBuf::from(path))
    }

    #[test]
    fn cache_path_normal() {
        let cwd = "/tmp/gitnu/opts_normal";
        let opts = _parse(&["ls-files"], cwd);
        test::mkdir(&cwd);
        test::sh(&["git", "init"], &cwd);
        assert_eq!(
            opts.cache_path(),
            Some(PathBuf::from(cwd).join(".git/gitnu.txt"))
        );
    }

    #[test]
    fn cache_path_diff_dir() {
        let cwd = "/tmp/gitnu/opts_diff_cwd";
        let repo = "/tmp/gitnu/opts_diff_repo";
        let opts = _parse(&["-C", repo, "ls-files"], cwd);
        test::mkdir(&cwd);
        test::mkdir(&repo);
        test::sh(&["git", "init"], &repo);
        test::sh(&["git", "init"], &cwd);
        assert_eq!(
            opts.cache_path(),
            Some(PathBuf::from(repo).join(".git/gitnu.txt"))
        );
    }
}