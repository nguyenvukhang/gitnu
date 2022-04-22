// use std::io::{self, Write};
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
    println!("====================");
    let mut git = Command::new("git");
    let git = git.args(["-C", "tests/repo"]);
    let git_status = git.arg("status");
    let output = git_status.output().expect("failed to execute command.");

    // TODO
    // learn string parsing
    // add numbers to lines with filenames inside

    // write entire output straight to shell
    // io::stdout().write_all(&output.stdout).unwrap();
    let stdout = &output.stdout;

    let string = String::from_utf8(stdout.to_vec()).unwrap();
    // let lines: Vec<String> = string.lines()
    let vec = string.lines();

    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let staged_start_regex = Regex::new(r"^Changes to be committed:.*").unwrap();
    let mut reading_staged = false;
    // let staged_index = 0;
    // let staged_changes: [&str; 100] = [""; 100];

    for i in vec {
        if i == "" {
            reading_staged = false;
        }
        if reading_staged {
            println!("read-staged: {}", i);
            continue;
        };
        if staged_start_regex.is_match(i) {
            println!("------------- Start reading staged changes");
            reading_staged = true;
            continue;
        };
        println!("line: {}", i)
    }

    println!("====================");

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
