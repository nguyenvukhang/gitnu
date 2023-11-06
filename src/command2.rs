use std::ffi::OsStr;
use std::process::Command;

#[derive(Debug)]
pub struct Command2 {
    pub inner: Command,
    pub hidden_args: Vec<usize>,
}

impl Command2 {
    pub fn new(program: &str) -> Self {
        Self {
            inner: Command::new(program),
            hidden_args: Vec::with_capacity(2),
        }
    }

    pub fn hidden_args<'a, I>(&mut self, args: I)
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut n = self.inner.get_args().len();
        for arg in args {
            self.inner.arg(arg);
            self.hidden_args.push(n);
            n += 1;
        }
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) {
        self.inner.arg(arg);
    }

    #[cfg(test)]
    pub fn get_args(&self) -> Vec<&str> {
        let mut hidden = self.hidden_args.clone();
        hidden.reverse();

        let mut i = 0usize;
        let mut args = vec![];

        for arg in self.inner.get_args() {
            if hidden.last() == Some(&i) {
                hidden.pop();
            } else if let Some(v) = arg.to_str() {
                args.push(v);
            }
            i += 1;
        }

        args
    }
}
