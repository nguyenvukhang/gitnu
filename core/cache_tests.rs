#[cfg(test)]
mod tests {
    use crate::test;
    use crate::{parser, Opts};
    use std::path::PathBuf;
    use std::{env, fs};

    pub fn parse(args: &[&str], path: &str) -> Opts {
        parser::parse(test::iter([&["gitnu"], args].concat()), path.into())
    }

    fn test_dir() -> String {
        env::var("GITNU_TEST_DIR").unwrap_or("/tmp/gitnu_rust".into())
    }

    #[test]
    fn cache_path_normal() {
        let cwd = format!("{}/opts_normal", test_dir());
        let opts = parse(&["ls-files"], &cwd);
        fs::create_dir_all(&cwd).ok();
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
        let cwd = format!("{}/opts_diff_cwd", test_dir());
        let repo = format!("{}/opts_diff_repo", test_dir());
        let opts = parse(&["-C", &repo, "ls-files"], &cwd);
        fs::create_dir_all(&cwd).ok();
        fs::create_dir_all(&repo).ok();
        test::sh_git(&["init"], &cwd);
        test::sh_git(&["init"], &repo);
        assert_eq!(
            opts.cache_path(),
            Some(PathBuf::from(repo).join(".git/gitnu.txt"))
        );
    }
}
