use crate::structs::Gitnu;
use std::collections::LinkedList;
use std::num::ParseIntError;
use std::process::Command;

fn get_range(arg: &str) -> Result<(usize, usize), ParseIntError> {
    let parts: Vec<&str> = arg.split("-").collect();
    let start: usize = parts[0].parse()?;
    let end: usize = parts[1].parse()?;
    match start < end {
        true => Ok((start, end)),
        false => Ok((end, start)),
    }
}

#[test]
fn test_get_range() {
    // equals
    assert_eq!(get_range(&format!("0-{}", usize::MAX)), Ok((0, usize::MAX)));
    assert_eq!(get_range("7-4"), Ok((4, 7)));
    assert_eq!(get_range("100-400"), Ok((100, 400)));

    // error handling
    assert!(get_range("-").is_err());
    assert!(get_range("--porcelain").is_err());
    assert!(get_range("--no-ff").is_err());

    // not equals
    assert_ne!(get_range("1.4"), Ok((1, 4)));
    assert_ne!(get_range("1_4"), Ok((1, 4)));
}

/// rargs is unprocessed and will be processed
/// largs will not be processed, only added to
pub fn read(
    mut rargs: LinkedList<String>,
    mut largs: LinkedList<String>,
    gitnu: &mut Gitnu,
) -> Command {
    while !rargs.is_empty() {
        let arg = rargs.pop_front();
        if arg.is_none() {
            continue;
        }
        let arg = arg.expect("Invalid argument");
        // positional argument is an integer
        if let Ok(v) = arg.parse::<usize>() {
            if let Ok(v) = gitnu.get_file(v) {
                largs.push_back(v);
            }
        }
        // positional argument is a number range
        else if let Ok((start, end)) = get_range(&arg) {
            for j in start..end + 1 {
                if let Ok(v) = gitnu.get_file(j) {
                    largs.push_back(v);
                }
            }
        }
        // just use the same value
        else {
            largs.push_back(arg);
        }
    }

    let mut cmd = Command::new(gitnu.cmd.get_program());
    cmd.args(largs);
    cmd
}
