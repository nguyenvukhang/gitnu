#[cfg(test)]
mod tests {
    use crate::test;
    use crate::{parser, Opts};
    use std::path::PathBuf;

    pub fn parse(args: &[&str], path: &str) -> Opts {
        parser::parse(test::iter([&["gitnu"], args].concat()), path.into())
    }

    #[test]
    fn cache_path_normal() {
        let cwd = "/tmp/gitnu/opts_normal";
        let opts = parse(&["ls-files"], cwd);
        test::mkdir(&cwd);
        test::sh_git(&["init"], &cwd);
        std::env::set_current_dir(&cwd).ok();
        assert_eq!(
            opts.cache_path(),
            Some(PathBuf::from(cwd).join(".git/gitnu.txt"))
        );
    }

    /// when gitnu is ran with a -C flag pointing to a different repo,
    /// read that repo's cache file instead
    #[test]
    fn cache_path_diff_dir() {
        let cwd = "/tmp/gitnu/opts_diff_cwd";
        let repo = "/tmp/gitnu/opts_diff_repo";
        let opts = parse(&["-C", repo, "ls-files"], cwd);
        test::mkdir(&cwd);
        test::mkdir(&repo);
        test::sh_git(&["init"], &cwd);
        test::sh_git(&["init"], &repo);
        assert_eq!(
            opts.cache_path(),
            Some(PathBuf::from(repo).join(".git/gitnu.txt"))
        );
    }
}
