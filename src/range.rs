use std::ffi::OsString;
use std::io::Error;
use std::io::ErrorKind::InvalidData;

/// parses two integers out of a range string
fn get_range(arg: &OsString) -> Result<(usize, usize), Error> {
    let err = || Error::new(InvalidData, "Unable to parse");
    let parse = |s: &str| s.parse().ok().ok_or(err());
    let arg = arg.to_str().ok_or(err())?;
    let (start, end) = arg.split_once("-").ok_or(err())?;
    let (s, e) = (parse(start)?, parse(end)?);
    return Ok(if s < e { (s, e) } else { (e, s) });
}

/// adds a range to an argument list
fn add_range(arg: OsString, args: &mut Vec<OsString>) {
    let mut use_range = |(start, end): (usize, usize)| {
        for i in start..end + 1 {
            args.push(OsString::from(i.to_string()));
        }
    };
    match get_range(&arg) {
        Ok(v) => use_range(v),
        _ => args.push(arg),
    }
}

/// unpack number ranges (returns a new vector)
/// 2-5   --->   2 3 4 5
pub fn load(args: Vec<OsString>) -> Vec<OsString> {
    let mut res: Vec<OsString> = Vec::new();
    for i in 1..args.len() {
        add_range(args[i].to_owned(), &mut res);
    }
    res
}
