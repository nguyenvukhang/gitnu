use std::io::Error;
use std::io::ErrorKind::InvalidData;

/// parses two integers out of a range string
fn get_range(arg: &str) -> Result<(usize, usize), Error> {
    let err = || Error::new(InvalidData, "Unable to parse range");
    let parse = |s: &str| s.parse().map_err(|_| err());
    let (start, end) = arg.split_once("-").ok_or(err())?;
    let (s, e) = (parse(start)?, parse(end)?);
    return Ok(if s < e { (s, e) } else { (e, s) });
}

/// adds a range to an argument list
fn add_range(arg: &str, args: &mut Vec<String>) {
    let mut use_range = |(start, end): (usize, usize)| {
        for i in start..end + 1 {
            args.push(i.to_string().to_owned());
        }
    };
    match get_range(&arg) {
        Ok(v) => use_range(v),
        _ => args.push(arg.to_owned()),
    }
}

/// unpack number ranges (returns a new vector)
/// 2-5   --->   2 3 4 5
pub fn load(args: Vec<String>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    for i in 1..args.len() {
        add_range(&args[i], &mut res);
    }
    res
}
