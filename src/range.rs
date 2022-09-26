use std::ffi::OsString;

// parses two integers out of a range string
fn get_range(arg: &OsString) -> Result<(usize, usize), ()> {
    let parts: Vec<&str> = arg.to_str().unwrap_or("").split("-").collect();
    if parts.len() < 2 {
        return Err(());
    }

    let start = parts[0].parse::<usize>();
    let end = parts[1].parse::<usize>();
    if start.is_err() || end.is_err() {
        return Err(());
    }

    let start: usize = start.unwrap();
    let end: usize = end.unwrap();
    match start < end {
        true => Ok((start, end)),
        false => Ok((end, start)),
    }
}

// adds a range to an argument list
fn add_range(arg: OsString, args: &mut Vec<OsString>) {
    if let Ok((start, end)) = get_range(&arg) {
        for i in start..end + 1 {
            args.push(i.to_string().into());
        }
    } else {
        args.push(arg);
    }
}

// unpack number ranges (returns a new vector)
// 2-5   --->   2 3 4 5
pub fn load(args: Vec<OsString>) -> Vec<OsString> {
    let mut res: Vec<OsString> = Vec::new();
    for i in 1..args.len() {
        add_range(args[i].to_owned(), &mut res);
    }
    res
}
