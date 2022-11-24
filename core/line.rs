pub trait Line {
    /// get contents after the last occurrence of the supplied byte
    fn after_last(&mut self, byte: u8);
    /// get contents after the first occurrence of the supplied byte
    fn after_first(&mut self, byte: u8);
    /// get contents after the first occurrence of the supplied byte
    fn after_first_sequence(&mut self, seq: &[u8]);
    /// repeatedly remove the first byte if it matches the supplied byte
    fn trim_left(&mut self, byte: u8);
    /// repeatedly remove the first byte if it passes the predicate
    fn trim_left_while<F: Fn(u8) -> bool>(&mut self, predicate: F);
    /// get contents after first occurrence of a whitespace byte
    fn after_first_whitespace(&mut self);
}

impl Line for &[u8] {
    fn after_last(&mut self, byte: u8) {
        while !self.is_empty() {
            *self = match find(self, byte) {
                None => break,
                Some(i) => &self[i + 1..],
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
            if let Some(i) = find(self, found[0]) {
                *self = &self[i + 1..];
                found = &found[1..];
            } else {
                found = seq
            }
        }
    }
    fn trim_left(&mut self, byte: u8) {
        while !self.is_empty() {
            *self = match self[0] {
                v if v == byte => &self[1..],
                _ => break,
            };
        }
    }
    fn trim_left_while<F: Fn(u8) -> bool>(&mut self, predicate: F) {
        while !self.is_empty() {
            *self = match predicate(self[0]) {
                true => &self[1..],
                _ => break,
            };
        }
    }
    fn after_first_whitespace(&mut self) {
        while !self.is_empty() {
            let end = self[0].is_ascii_whitespace();
            *self = &self[1..];
            if end {
                break;
            }
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
