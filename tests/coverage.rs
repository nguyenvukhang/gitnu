use std::collections::HashSet;

const REQUIRED: &[&str] = &[
    "add_unindexed_number",
    "change_cwd_after_status",
    "dont_create_cache_file_without_repo",
    "not_from_git_root",
    "range_overlap",
    "reset_unindexed_number",
    "staging_files_with_filename",
    "staging_files_with_numbers",
    "stop_after_double_dash",
    "support_aliases",
    "untracked_files",
    "zero_filename",
    "zero_handling",
    "detached_head",
    "every_git_state",
    "merge_conflict",
    "handle_capital_c_flag",
    "skip_short_flags",
];

#[cfg(test)]
fn get_required_tests() -> HashSet<String> {
    HashSet::from_iter(REQUIRED.iter().map(|v| v.to_string()))
}

#[cfg(test)]
fn get_implemented_tests() -> Vec<String> {
    fn inner() -> Option<Vec<String>> {
        use std::io::{BufRead, BufReader};
        use std::process::{Command, Stdio};
        let mut cargo = Command::new("cargo");
        cargo.args(["test", "--", "--list", "--format=terse"]);
        let mut cargo = cargo.stdout(Stdio::piped()).spawn().ok()?;
        let v = BufReader::new(cargo.stdout.as_mut()?)
            .lines()
            .filter_map(|v| v.ok())
            .filter_map(|v| v.rsplit_once(':').map(|v| v.0.to_string()))
            .collect();
        cargo.wait().ok()?;
        Some(v)
    }
    inner().unwrap_or(vec![])
}

#[cfg(test)]
fn check_coverage(prefix: &str, implemented: &Vec<String>) {
    let mut required = get_required_tests();
    implemented.iter().for_each(|v| {
        if let Some(test) = v.strip_prefix(prefix) {
            required.remove(test);
        }
    });
    if !required.is_empty() {
        panic!(
            "\n\nNot all commands are tested.\n\nUntested subcommands:\n{:?}
            \n({} untested)\n\n",
            required,
            required.len()
        );
    };
}

#[test]
fn test() {
    let implemented = get_implemented_tests();
    check_coverage("normal::", &implemented);
}
