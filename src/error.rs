use std::any::Any;
use std::io;

#[derive(Debug)]
pub enum Error {
    InvalidCache,
    NotGitCommand,
    NotGitRepository,
    NotImplemented,
    Io(io::Error),
    ThreadError(Box<dyn Any + Send + 'static>),
}

impl PartialEq for Error {
    fn eq(&self, rhs: &Error) -> bool {
        let lhs = self;
        use Error::*;
        match (lhs, rhs) {
            (NotGitRepository, NotGitRepository) => true,
            (NotImplemented, NotImplemented) => true,
            (InvalidCache, InvalidCache) => true,
            (NotGitCommand, NotGitCommand) => true,
            (Io(lhs), Io(rhs)) => lhs.kind() == rhs.kind(),
            (ThreadError(_), ThreadError(_)) => true,
            _ => false,
        }
    }
}

#[macro_export]
macro_rules! error {
    ($enum:ident, $from:ty) => {
        impl From<$from> for Error {
            fn from(err: $from) -> Self {
                Self::$enum(err)
            }
        }
    };
    ($enum:ident) => {
        Err(Error::$enum)
    };
}

error!(Io, io::Error);
error!(ThreadError, Box<dyn Any + Send + 'static>);
