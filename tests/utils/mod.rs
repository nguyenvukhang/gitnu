#![allow(dead_code)]

use gitnu;
use std::collections::LinkedList;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn get_args(cmd: &Command) -> Vec<String> {
    cmd.get_args().map(|a| String::from(a.to_str().unwrap())).collect()
}

pub fn cli(v: Vec<&str>) -> LinkedList<String> {
    let mut args: LinkedList<String> = LinkedList::new();
    for i in v {
        args.push_back(String::from(i));
    }
    args
}

/// test CLI input against gitnu's output args
pub fn test(input: Vec<&str>, expected: Vec<&str>) {
    let cwd = std::env::current_dir().unwrap();
    let g = gitnu::core(cli(input), cwd);
    let g = get_args(&g.cmd);
    assert_eq!(g, expected);
}

pub fn get_stdout(cmd: &mut Command) -> String {
    let out = cmd.output().expect("unable to get_stdout");
    let out = String::from_utf8_lossy(&out.stdout);
    String::from(out)
}

pub fn multiline(vec: &[&str]) -> String {
    vec.join("\n")
}

pub struct Git {
    cmd_str: String,
    cwd: String,
}
impl Git {
    pub fn new(cmd: &str, cwd: &str) -> Git {
        let cmd_str = String::from(cmd);
        Git { cwd: String::from(cwd), cmd_str }
    }
    pub fn cmd(&self) -> Command {
        let mut cmd = Command::new(&self.cmd_str);
        cmd.current_dir(&self.cwd);
        cmd
    }
    pub fn git(&mut self, args: &[&str]) -> String {
        let c = &mut self.cmd();
        c.args(args);
        let s = get_stdout(c);
        print!("{}", s);
        s
    }
    pub fn run(&mut self, args: &[&str]) -> String {
        let mut c = Command::new(args[0]);
        c.current_dir(&self.cwd);
        c.args(&args[1..]);
        get_stdout(&mut c)
    }
    // to init git and gitnu at the same time
    pub fn pair(cmd: [&str; 2], cwd: &str) -> (Git, Git) {
        return (Git::new(cmd[0], cwd), Git::new(cmd[1], cwd));
    }
    // to just use git and gitnu immediately
    pub fn gpair(cwd: &str) -> (Git, Git) {
        Git::pair(["git", "gitnu"], cwd)
    }

    // git commands
    pub fn init(&mut self) -> String {
        fs::create_dir_all(&self.cwd)
            .expect(&format!("unable to mkdir: {}", &self.cwd));
        self.git(&["init"])
    }
    pub fn add(&mut self, file: &str) -> String {
        self.git(&["add", file])
    }
    pub fn reset(&mut self, file: &str) -> String {
        self.git(&["reset", file])
    }
    pub fn status(&mut self) -> String {
        self.git(&["status"])
    }
    pub fn commit(&mut self) -> String {
        self.git(&["commit", "-m", "commit_message"])
    }
    pub fn xargs(&mut self, args: &[&str]) -> String {
        self.git(args)
    }

    // generic shell commands
    pub fn touch(&mut self, file: &str) {
        self.run(&["touch", file]);
    }
    pub fn append(&mut self, file: &str, text: &str) {
        let p = Path::new(&self.cwd).join(file);
        let contents = fs::read_to_string(&p)
            .expect(&format!("unable to modify file {}", file));
        fs::write(p, &format!("{}\n{}", contents, text))
            .expect(&format!("unable to modify file {}", file));
    }
    pub fn modify(&mut self, file: &str) {
        let p = Path::new(&self.cwd).join(file);
        let contents = fs::read_to_string(&p)
            .expect(&format!("unable to modify file {}", file));
        fs::write(p, &format!("{}{}", "__edit__", contents))
            .expect(&format!("unable to modify file {}", file));
    }
    pub fn ls(&mut self) -> String {
        self.run(&["ls"])
    }
}
