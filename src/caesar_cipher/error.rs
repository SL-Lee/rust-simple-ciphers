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
                write!(f, "`key` must be a positive integer between 1 and 25")
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidKey => {
                write!(f, "`key` must be a positive integer between 1 and 25")
            }
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidKey,
}
