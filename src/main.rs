use std::io::{self, Write};
use regex::Regex;
use std::process::Command;

/* base implementation targets
 *   - git status
 *   - git add
 *   - git reset
 *   - git checkout
 */

// write a test-repo generator with bash
// use this for unit testing

// write a test for git status

fn main() {
    // read the contents of git status and write it into
    // .git/gitnumber.txt
    let mut git = Command::new("git");
    let git = git.args(["-C", "tests/repo"]);
    let git_status = git.arg("status");
    let output = git_status.output().expect("failed to execute command.");

    // TODO
    // learn string parsing
    // add numbers to lines with filenames inside

    // write entire output straight to shell
    println!("[ ORIGINAL ]");
    io::stdout().write_all(&output.stdout).unwrap();
    println!("[ NUMBERED ]");
    let stdout = &output.stdout;

    let string = String::from_utf8(stdout.to_vec()).unwrap();
    // let lines: Vec<String> = string.lines()
    let vec = string.lines();

    let staged_start_regex = Regex::new(r"^Changes to be committed:$").unwrap();
    let unstaged_start_regex = Regex::new(r"^Changes not staged for commit:$").unwrap();
    let untracked_start_regex = Regex::new(r"^Untracked files:$").unwrap();
    let mut reading_staged = false;
    let mut reading_unstaged = false;
    let mut reading_untracked = false;
    let mut mi = 0;
    let mut numbered_changes: [(&str, &str); 100] = [("", ""); 100];
    // let format = String::from("{}{}");
    fn gitline(mi: usize, s: &str) {
        println!("{}{}", mi, s)
    }

    for i in vec {
        if i == "" {
            reading_staged = false;
            reading_unstaged = false;
            reading_untracked = false;
        }
        if reading_staged {
            gitline(mi, i);
            numbered_changes[mi] = ("staged", i);
            mi += 1;
            continue;
        };
        if reading_unstaged {
            gitline(mi, i);
            numbered_changes[mi] = ("unstaged", i);
            mi += 1;
            continue;
        };
        if reading_untracked {
            gitline(mi, i);
            numbered_changes[mi] = ("untracked", i);
            mi += 1;
            continue;
        };
        if staged_start_regex.is_match(i) {
            reading_staged = true;
            continue;
        };
        if unstaged_start_regex.is_match(i) {
            reading_unstaged = true;
            continue;
        };
        if untracked_start_regex.is_match(i) {
            reading_untracked = true;
            continue;
        };
        println!("line: {}", i)
    }

    // println!("====================");
    // println!("POST PROCESSING");
    //
    // for i in 0..100 {
    //     if numbered_changes[i].0 != "" {
    //         let (t, s) = numbered_changes[i];
    //         if t == "staged" || t == "unstaged" {
    //             match s.split_once(":") {
    //                 Some((_, value)) => {
    //                     println!("filename: {}", value.trim());
    //                 }
    //                 None => {
    //                     println!("expected line to have a colon");
    //                 }
    //             };
    //             continue;
    //         }
    //         println!("{}: {}", i, s);
    //     }
    // }

    // read the contents of gitnumber.txt and use it to do cool stuff
}

/* far implementation targets
 *   - pipe a series of files into an arbitrary command
 *     gitn -c rm 2  # deletes file 2
 *     gitn -c vim 3 # opens file 3 in vim
 */

/* coverage implementation targets
 *   - git status -s
 *   - git diff
 */
