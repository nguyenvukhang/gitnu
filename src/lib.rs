use std::io::{BufRead, BufReader, LineWriter, Write};
use std::{env::current_dir as cwd, path::PathBuf, process::Command as C};
use std::{fs::File, process::Stdio};

#[derive(Debug, PartialEq)]
pub enum Op {
    Status(bool),    // gitnu status (true: normal, false: short)
    Number(PathBuf), // gitnu -c nvim 2 / gitnu add 2-4
}

pub struct Opts {
    pub op: Op,
    pub cwd: PathBuf,
}

impl Opts {
    pub fn cache(&self, create: u8) -> Option<File> {
        let (g, t, f) = ("git", ["rev-parse", "--git-dir"], "gitnu.txt");
        let t = C::new(g).args(t).current_dir(&self.cwd).output().ok()?.stdout;
        let f = PathBuf::from(String::from_utf8_lossy(&t).trim_end()).join(f);
        if create.eq(&1) { File::create(f) } else { File::open(f) }.ok()
    }
}

pub fn parse(args: Vec<String>) -> (Vec<String>, Opts) {
    let (mut res, mut iter, p) = (Vec::new(), args.iter(), |a: &str| a.into());
    let mut o = Opts { cwd: cwd().unwrap_or(p(".")), op: Op::Number(p("git")) };
    iter.next();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "status" => o.op = Op::Status(true),
            "--short" | "-s" | "--porcelain" => match o.op {
                Op::Status(_) => o.op = Op::Status(false),
                _ => (),
            },
            "-c" | "-C" => {
                if let Some(v) = iter.next() {
                    match arg.as_str() {
                        "-c" => o.op = Op::Number(PathBuf::from(v)),
                        _ => o.cwd = PathBuf::from(v),
                    }
                }
                continue;
            }
            _ => (),
        }
        res.push(arg.to_string());
    }
    (res, o)
}

fn get_range(arg: &str) -> Option<[usize; 2]> {
    arg.parse().map(|v| Some([v, v])).unwrap_or_else(|_| {
        let (a, b) = arg.split_once("-")?;
        let a = a.parse().ok()?;
        let b = b.parse().unwrap_or(a);
        Some(if a < b { [a, b] } else { [b, a] })
    })
}

pub fn load(args: Vec<String>, o: &Opts) -> Vec<PathBuf> {
    let (c, mut sk) = (|v| BufReader::new(v).lines().filter_map(|v| v.ok()), 0);
    let c: Vec<String> = o.cache(0).map(|v| c(v).collect()).unwrap_or_default();
    args.iter().fold(Vec::new(), |mut r, a| {
        let isf = a.starts_with('-') && !a.starts_with("--"); // is short flag
        let [s, e] = get_range(a).unwrap_or([0, 0]);
        match sk == 1 || isf || [s, e] == [0, 0] {
            true => r.push(PathBuf::from(a)),
            false => (s..e + 1)
                .map(|n| (n.checked_sub(1).map(|v| c.get(v)), n.to_string()))
                .for_each(|(o, s)| r.push(o.flatten().unwrap_or(&s).into())),
        }
        sk = if isf { 1 } else { 0 };
        return r;
    })
}

fn uncolor(f: &str) -> String {
    f.replace("\x1b[31m", "").replace("\x1b[32m", "").replace("\x1b[m", "")
}

pub fn status(args: &Vec<PathBuf>, o: Opts) -> Option<()> {
    let (c, mut su) = (&mut 1, false);
    let (mut g, cs) = (C::new("git"), ["-c", "color.status=always"]);
    g.current_dir(&o.cwd).args(cs).args(args).stdout(Stdio::piped());
    let (mut w, mut g) = (LineWriter::new(o.cache(1)?), g.spawn().ok()?);
    let b = BufReader::new(g.stdout.as_mut()?).lines().filter_map(|v| v.ok());
    b.filter_map(|l| {
        su |= l.contains("Untracked files:");
        match (&o.op, l.starts_with('\t')) {
            (Op::Status(true), false) => println!("{}", l),
            (Op::Status(true), true) => {
                println!("{}{}", *c, l);
                *c += 1;
                let f = uncolor(&l);
                let mut f = f.split_once('\t')?.1;
                f = if su { f } else { f.split_once(':')?.1.trim_start() };
                return Some(o.cwd.join(f));
            }
            _ => {
                println!("{: <3}{}", *c, l);
                *c += 1;
                return Some(o.cwd.join(&uncolor(&l)[3..]));
            }
        };
        return None;
    })
    .for_each(|v| writeln!(w, "{}", v.display()).unwrap_or(()));
    g.wait().ok();
    w.flush().ok()
}

pub fn core(args: Vec<String>) -> (Vec<PathBuf>, Opts) {
    let (args, opts) = parse(args);
    (load(args, &opts), opts)
}

pub fn run(a: Vec<PathBuf>, opts: Opts) -> Option<()> {
    let sp = |c| C::new(c).args(&a).spawn().ok()?.wait().map(|_| ()).ok();
    match opts.op {
        Op::Status(_) => status(&a, opts),
        Op::Number(c) => sp(c),
    }
}

#[cfg(test)]
mod tests;
