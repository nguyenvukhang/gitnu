// use std::io::{self, Write};
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

    for i in vec {
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
