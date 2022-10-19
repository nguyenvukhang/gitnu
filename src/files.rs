use crate::Opts;
use std::io::{BufRead, BufReader};
use std::{cmp::max, fs::File, path::PathBuf};

fn get_range(arg: &str) -> Option<[usize; 2]> {
    arg.parse().map(|v| Some([v, v])).unwrap_or_else(|_| {
        let (a, b) = arg.split_once("-")?;
        let a = a.parse().ok()?;
        let b = b.parse().unwrap_or(a);
        Some(if a < b { [a, b] } else { [b, a] })
    })
}

pub fn load(args: Vec<String>, opts: &Opts) -> Vec<PathBuf> {
    let mut r: Vec<PathBuf> = Vec::new();
    let c = || -> Option<Vec<String>> {
        let f = BufReader::new(File::open(opts.cache_file()?).ok()?).lines();
        Some(f.filter_map(|v| v.ok()).collect())
    };
    let (c, mut skip) = (c().unwrap_or(Vec::new()), false);
    args.iter().for_each(|a| {
        let isf = a.starts_with('-') && !a.starts_with("--"); // is short flag
        let [s, e] = get_range(a).unwrap_or([0, 0]);
        if skip || isf || [s, e] == [0, 0] {
            r.push(PathBuf::from(a));
        } else {
            (s..e + 1)
                .map(|n| (n > 0, n.to_string(), c.get(max(n, 1) - 1)))
                .map(|(z, s, g)| if z { g.unwrap_or(&s) } else { &s }.into())
                .for_each(|v| r.push(v));
        }
        skip = isf;
    });
    return r;
}
