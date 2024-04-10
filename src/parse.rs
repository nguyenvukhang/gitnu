use std::collections::HashMap;

use crate::prelude::*;

type Aliases = HashMap<String, String>;

/// Parses a string into an inclusive range.
/// "5"   -> Some([5, 5])
/// "2-6" -> Some([2, 6])
/// "foo" -> None
pub fn parse_range(arg: &str) -> Option<(usize, usize)> {
    if let Ok(single) = arg.parse::<usize>() {
        Some((single, single))
    } else {
        let (a, b) = arg.split_once('-')?;
        let a = a.parse::<usize>().ok()?;
        let b = b.parse::<usize>().ok()?;
        Some((a.min(b), a.max(b)))
    }
}

/// Parses ALL args, including the bin path.
pub fn parse<A: ArgHolder>(
    args: &[String],
    aliases: Aliases,
    cache: Cache,
    mut argh: A,
) -> (A, Option<GitCommand>) {
    let mut git_cmd = None::<GitCommand>;

    #[cfg(not(test))]
    if atty::is(atty::Stream::Stdout) {
        argh.add_args(["-c", "color.ui=always"]);
    }

    let mut args = &args[1..]; // skip the binary path

    // BEFORE git command is found
    while !args.is_empty() {
        let arg = args[0].as_str();
        args = &args[1..];
        match GitCommand::from_arg(&aliases, arg) {
            Some(v) => {
                git_cmd = Some(v);
                argh.add_arg(arg);
                break;
            }
            _ => argh.add_arg(arg),
        }
    }

    // AFTER git command is looked for/found
    if let None = git_cmd {
        // add remaining args and send it
        argh.add_args(args);
        return (argh, git_cmd);
    }

    for i in 0..args.len() {
        let arg = args[i].as_str();
        let git_cmd = git_cmd.as_mut().unwrap();
        match git_cmd {
            GitCommand::Status(ref mut v) => match arg {
                "--short" | "-s" | "--porcelain" => v.short(),
                _ => {}
            },
            _ => {}
        };
        let skip = i > 0 && git_cmd.skip_next_arg(&args[i - 1]);
        match (skip, parse_range(&arg)) {
            (false, Some((start, end))) if end <= MAX_CACHE_SIZE => {
                for i in start..end + 1 {
                    cache.load(i, &mut argh);
                }
            }
            _ => argh.add_arg(arg),
        }
    }
    (argh, git_cmd)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    fn parse(args: &[&str]) -> Vec<String> {
        let mut args = string_vec(args);
        args.insert(0, "git".to_string());
        super::parse(&args, Aliases::default(), Cache::default(), vec![]).0
    }

    macro_rules! test {
        ($name:ident, $input_args:expr, $output_args:expr) => {
            #[test]
            fn $name() {
                let received_args = parse(&$input_args);
                let expected_args = string_vec($output_args);
                assert_eq!(received_args, expected_args);
            }
        };
    }

    test!(test_single, ["add", "1"], ["add", "1"]);
    test!(test_range, ["add", "2-4"], ["add", "2", "3", "4"]);
    test!(test_mix, ["add", "8", "2-4"], ["add", "8", "2", "3", "4"]);

    // Gitnu will not seek to interfere with these cases smartly.
    test!(
        test_overlap,
        ["add", "3-5", "2-4"],
        ["add", "3", "4", "5", "2", "3", "4"]
    );

    // anything after `--` will also be processed. This is for commands
    // like `git reset` which requires pathspecs to appear after --.
    test!(
        test_double_dash,
        ["add", "3-5", "--", "2-4"],
        ["add", "3", "4", "5", "--", "2", "3", "4"]
    );

    test!(test_zeros_1, ["add", "0"], ["add", "0"]);
    test!(test_zeros_2, ["add", "0-1"], ["add", "0", "1"]);
    test!(test_zeros_3, ["add", "0-0"], ["add", "0"]);

    // Filenames containing dashed dates
    test!(test_date_filename, ["add", "2021-01-31"], ["add", "2021-01-31"]);
}
