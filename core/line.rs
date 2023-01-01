pub trait Line {
    /// get contents after the last occurrence of the supplied byte
    fn after_last(&mut self, byte: u8);

    /// get contents after the first occurrence of the supplied byte
    fn after_first(&mut self, byte: u8);

    /// get contents after the first occurrence of the supplied byte
    fn after_first_sequence(&mut self, seq: &[u8]);

    /// repeatedly remove the first byte if it passes the predicate
    fn trim_left_while<F: Fn(u8) -> bool>(&mut self, predicate: F);
}

impl Line for &[u8] {
    fn after_last(&mut self, byte: u8) {
        while !self.is_empty() {
            match find(self, byte) {
                None => break,
                Some(i) => *self = &self[i + 1..],
            }
        }
    }

    fn after_first(&mut self, byte: u8) {
        if let Some(i) = find(self, byte) {
            *self = &self[i + 1..]
        }
    }

    fn after_first_sequence(&mut self, seq: &[u8]) {
        let mut found = seq;
        while !found.is_empty() && !self.is_empty() {
            if found[0] == self[0] {
                *self = &self[1..];
                found = &found[1..];
                continue;
            } else {
                *self = &self[1..];
                found = seq;
            }
        }
    }

    fn trim_left_while<F: Fn(u8) -> bool>(&mut self, predicate: F) {
        while !self.is_empty() {
            match predicate(self[0]) {
                true => *self = &self[1..],
                _ => break,
            };
        }
    }
}

/// Removes all ANSI color codes
pub fn uncolor(src: &str) -> Vec<u8> {
    let (mut src, mut dst) = (src.as_bytes(), vec![]);
    while !src.is_empty() {
        match find(src, b'\x1b') {
            None => break,
            Some(i) => {
                dst.extend(&src[..i]);
                src = &src[i..];
            }
        }
        match find(src, b'm') {
            None => break,
            Some(i) => src = &src[i + 1..],
        }
    }
    dst.extend(src);
    dst
}

/// finds the first occurrence of needle byte in an array of bytes
fn find(hay: &[u8], needle: u8) -> Option<usize> {
    hay.iter().position(|&b| b == needle)
}

#[cfg(test)]
macro_rules! test {
    ($fun:expr, $input:expr, $expected:expr) => {
        let line = &mut $input.as_bytes();
        $fun(line);
        assert_eq!(std::str::from_utf8(line), Ok($expected));
    };
}

#[test]
fn after_last_test() {
    let f = |v: &mut &[u8]| v.after_last(b'x');
    test!(f, "fooxbarx", "");
    test!(f, "fooxbar", "bar");
    test!(f, "xfooxbar", "bar");
    test!(f, "", "");
}

#[test]
fn after_first_test() {
    let f = |v: &mut &[u8]| v.after_first(b'x');
    test!(f, "fooxbarx", "barx");
    test!(f, "fooxbar", "bar");
    test!(f, "xfooxbar", "fooxbar");
    test!(f, "", "");
}

#[test]
fn after_first_sequence_test() {
    let f = |v: &mut &[u8]| v.after_first_sequence(b"foo");
    test!(f, "xfooxbar", "xbar");
    test!(f, "foo bar", " bar");
    test!(f, "bar foo", "");
    test!(f, "f o o foobar", "bar");
    test!(f, "", "");
}

#[test]
fn trim_left_while() {
    let f = |v: &mut &[u8]| v.trim_left_while(|b| b.is_ascii_whitespace());
    test!(f, "  foo  ", "foo  ");
    test!(f, "\t\nfoo", "foo");
    test!(f, "\x0C\t\n\rfoo", "foo");
    test!(f, "", "");
}
