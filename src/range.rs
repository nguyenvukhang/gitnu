fn get_range(arg: &str) -> Option<(usize, usize)> {
    let (start, end) = arg.split_once("-")?;
    let int = |s: &str| s.parse().ok();
    let (s, e) = (int(start)?, int(end)?);
    Some(if s < e { (s, e) } else { (e, s) })
}

fn add_range(arg: &str, args: &mut Vec<String>) {
    let mut push_range = |(start, end): (usize, usize)| {
        for i in start..end + 1 {
            args.push(i.to_string());
        }
    };
    match get_range(&arg) {
        Some(v) => push_range(v),
        _ => args.push(arg.to_string()),
    }
}

pub fn load(args: Vec<String>) -> Vec<String> {
    let mut res = Vec::new();
    let mut it = args.iter();
    while let Some(arg) = it.next() {
        add_range(&arg, &mut res);
    }
    res
}
