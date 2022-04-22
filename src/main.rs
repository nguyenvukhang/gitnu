use regex::Regex;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::vec::Vec;
// use std::str::Lines;
//
// fn main() -> std::io::Result<()> {
//     let mut file = File::create("foo.txt")?;
//     file.write_all(b"Hello, world!")?;
//     Ok(())
// }

/* base implementation targets
 *   - git status
 *   - git add
 *   - git reset
 *   - git checkout
 */

// write a test-repo generator with bash
// use this for unit testing

// write a test for git status
//
const LIMIT: usize = 1000;

fn save_cache(gw: Vec<&str>) {
    let ran_from = "tests/repo/some/thing/is/up";
    let mut git = Command::new("git");
    let git = git
        .args(["-C", ran_from])
        .arg("rev-parse")
        .arg("--show-toplevel");
    let output = git.output().expect("failed to execute command.").stdout;
    let mut git_root = String::from_utf8(output.to_vec())
        .unwrap()
        .trim()
        .to_owned();

    let filename: &str = "/.git/gitnumber.txt";
    git_root.push_str(filename);

    println!("ran from dir: {}", ran_from);
    println!("full git root: {}", git_root);
    // let asdf = String::from_utf8(gonna_write.to_vec());
    // fs::write(git_root, "yellow").expect("Unable to write file");
    // print!("{}", &output);

    let mut file = fs::File::create(git_root).expect("could not create file.");
    write!(file, "{}", gw.join("\n")).expect("could not write file");
}

fn main() {
    // read the contents of git status and write it into
    // .git/gitnumber.txt
    println!("---------------------------------------------------------");
    let mut git = Command::new("git");
    let git = git.args(["-C", "tests/repo"]);
    let git_status = git.arg("-c").arg("color.status=always").arg("status");

    let output = git_status.output().expect("failed to execute command.");

    // TODO
    // learn string parsing
    // add numbers to lines with filenames inside

    // write entire output straight to shell
    // println!("[ ORIGINAL ]");
    // io::stdout().write_all(&output.stdout).unwrap();
    println!("[ NUMBERED ]");
    let stdout = &output.stdout;

    let string = String::from_utf8(stdout.to_vec()).unwrap();
    // let lines: Vec<String> = string.lines()
    let mut vec = string.lines();

    let staged_start_regex = Regex::new(r"^Changes to be committed:$").unwrap();
    let unstaged_start_regex = Regex::new(r"^Changes not staged for commit:$").unwrap();
    let untracked_start_regex = Regex::new(r"^Untracked files:$").unwrap();
    let mut reading_staged = false;
    let mut reading_unstaged = false;
    let mut reading_untracked = false;
    let mut mi = 1;
    let mut wi = 0;
    let mut numbered_changes: [(&str, &str); LIMIT] = [("", ""); LIMIT];
    let mut gonna_write: [&str; LIMIT] = [""; LIMIT];
    let mut gw: Vec<&str> = vec![];

    fn gitline(mi: usize, s: &str) {
        println!("{}{}", mi, s);
    }

    fn sanitize_string(s: String)-> String {
        // s = str::replace(&s, "!", "?");
        return s
    }

    for i in vec {
        gonna_write[wi] = i;
        gw.push(i);
        wi += 1;
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
            println!("{}", i);
            continue;
        };
        if unstaged_start_regex.is_match(i) {
            reading_unstaged = true;
            println!("{}", i);
            continue;
        };
        if untracked_start_regex.is_match(i) {
            reading_untracked = true;
            println!("{}", i);
            continue;
        };
        println!("{}", i);
    }
    save_cache(gw);

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
