use std::result as core;

pub type Result<T> = core::Result<T, String>;

pub trait StringError<T, E> {
    fn serr(self, err_msg: &str) -> Result<T>;
    fn clear(self) -> core::Result<(), E>;
}

impl<T, E> StringError<T, E> for core::Result<T, E> {
    fn serr(self, err_msg: &str) -> Result<T> {
        self.map_err(|_| err_msg.to_string())
    }
    fn clear(self) -> core::Result<(), E> {
        return self.map(|_| ());
    }
}

pub fn err<T>(msg: &str) -> Result<T> {
    Err(msg.to_string())
}
