use std::{error, fmt};

pub struct Error {
    kind: ErrorKind,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error { kind }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl error::Error for Error {}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidKey => {
                write!(
                    f,
                    "`key` must be a string of integers of range 0 to n - 1, \
                    where n is the length of the key itself, in an arbitrary \
                    order"
                )
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidKey => {
                write!(
                    f,
                    "`key` must be a string of integers of range 0 to n - 1, \
                    where n is the length of the key itself, in an arbitrary \
                    order"
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidKey,
}
