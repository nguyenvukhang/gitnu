use std::io::{BufRead, BufReader, LineWriter, Read, Write};
use std::{env::current_dir, fs::File, process::Stdio};
use std::{path::PathBuf, process::Command};
mod git_cmd;

#[derive(Debug, PartialEq)]
pub enum Op {
    Status(bool),   // bool: { true: normal, false: short }
    Number(String), // PathBuf contains command
    Unset,
}

pub struct Opts {
    op: Op,
    cwd: PathBuf,
}

impl Opts {
    pub fn cache(&self, create: bool) -> Option<File> {
        let mut git = Command::new("git");
        git.args(["rev-parse", "--git-dir"]).current_dir(&self.cwd);
        let r = git.output().ok()?;
        let p = match r.status.success() {
            false => return None,
            true => PathBuf::from(String::from_utf8_lossy(&r.stdout).trim()),
        }
        .join("gitnu.txt");
        if create { File::create(p) } else { File::open(p) }.ok()
    }
    pub fn set_once(&mut self, op: Op) {
        match (&self.op, &op) {
            (Op::Unset, _) => self.op = op,
            (Op::Status(true), Op::Status(false)) => self.op = op,
            _ => (),
        }
    }
}

pub fn parse(
    args: impl Iterator<Item = String>,
) -> (impl Iterator<Item = String>, Opts) {
    let git_cmd = git_cmd::set();
    let mut iter = args.skip(1).peekable();
    let mut res = Vec::new();
    let mut opts =
        Opts { op: Op::Unset, cwd: current_dir().unwrap_or(".".into()) };
    while let Some(mut arg) = iter.next() {
        if opts.op == Op::Unset && git_cmd.contains(&arg) {
            match arg.as_str() {
                "status" => opts.set_once(Op::Status(true)),
                _ => opts.set_once(Op::Number("git".into())),
            }
        }
        match (iter.peek(), arg.as_str()) {
            (_, "status") => opts.set_once(Op::Status(true)),
            (_, "--short" | "-s") => opts.set_once(Op::Status(false)),
            (_, "--porcelain") => opts.set_once(Op::Status(false)),
            (Some(cwd), "-C") => opts.cwd = cwd.into(),
            (Some(cmd), "-x") => {
                opts.set_once(Op::Number(cmd.into()));
                iter.next();
                continue;
            }
            _ => (),
        }
        res.push(std::mem::take(&mut arg));
    }
    opts.set_once(Op::Number("git".into()));
    (res.into_iter(), opts)
}

fn lines<R: Read>(src: R) -> impl Iterator<Item = String> {
    BufReader::new(src).lines().filter_map(|v| v.ok())
}

pub fn load(args: impl Iterator<Item = String>, opts: &Opts) -> Vec<PathBuf> {
    fn get_range(arg: &str) -> Option<[usize; 2]> {
        arg.parse().map(|v| Some([v, v])).unwrap_or_else(|_| {
            let (a, b) = arg.split_once("-")?;
            let a: usize = a.parse().ok()?;
            Some(b.parse().map(|b| [a.min(b), a.max(b)]).unwrap_or([a, a]))
        })
    }
    let (mut skip, mut bypass) = (false, false);
    let c: Vec<String> =
        opts.cache(false).map(|v| lines(v).collect()).unwrap_or_default();
    args.fold(Vec::new(), |mut r, a| {
        bypass |= a.eq("--");
        let isf = a.starts_with('-') && !a.starts_with("--"); // is short flag
        match (bypass, !skip && !isf, get_range(&a)) {
            (false, true, Some([s, e])) => (s..e + 1)
                .map(|n| (n.checked_sub(1).map(|v| c.get(v)), n.to_string()))
                .for_each(|(o, s)| r.push(o.flatten().unwrap_or(&s).into())),
            _ => r.push(PathBuf::from(a)),
        }
        skip = isf;
        r
    })
}

pub fn status(args: &Vec<PathBuf>, o: Opts, is_normal: bool) -> Option<()> {
    const C: [&str; 3] = ["\x1b[31m", "\x1b[32m", "\x1b[m"];
    let rmc = |v: &str| v.replace(C[0], "").replace(C[1], "").replace(C[2], "");
    let mut count = 1;
    let mut su = false;
    let mut git = Command::new("git");
    git.args(["-c", "color.status=always"]).args(args).stdout(Stdio::piped());
    let mut git = git.spawn().ok()?;
    let b = lines(git.stdout.as_mut()?);
    let mut writer = o.cache(true).map(LineWriter::new);
    b.filter_map(|line| {
        su |= line.contains("Untracked files:");
        match (is_normal, line.starts_with('\t')) {
            (true, false) => println!("{}", line),
            (true, true) => {
                println!("{}{}", count, line);
                count += 1;
                let f = rmc(&line.trim_start_matches('\t'));
                let f = if su { &f } else { f.split_once(':')?.1.trim_start() };
                return Some(o.cwd.join(f));
            }
            _ => {
                println!("{: <3}{}", count, line);
                count += 1;
                return Some(o.cwd.join(&rmc(&line)[3..]));
            }
        };
        return None;
    })
    .for_each(|line| {
        writer.as_mut().map(|lw| writeln!(lw, "{}", line.display()));
    });
    git.wait().ok();
    writer.map(|mut lw| lw.flush().ok()).flatten()
}

pub fn core(args: impl Iterator<Item = String>) -> (Vec<PathBuf>, Opts) {
    let (args, opts) = parse(args);
    match opts.op {
        Op::Status(_) => (args.map(PathBuf::from).collect(), opts),
        _ => (load(args, &opts), opts),
    }
}

pub fn run(a: Vec<PathBuf>, opts: Opts) -> Option<()> {
    let sp = |c| Command::new(c).args(&a).spawn().ok()?.wait().map(|_| ()).ok();
    match opts.op {
        Op::Status(normal) => status(&a, opts, normal),
        Op::Number(cmd) => sp(cmd),
        Op::Unset => None,
    }
}

#[cfg(test)]
mod tests;
