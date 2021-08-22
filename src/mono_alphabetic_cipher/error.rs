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
                write!(f, "`key` must contain all 26 unique alphabets")
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidKey => {
                write!(f, "`key` must contain all 26 unique alphabets")
            }
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidKey,
}