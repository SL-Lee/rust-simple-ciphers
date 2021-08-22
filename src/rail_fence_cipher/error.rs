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
                write!(f, "`key` must be a positive integer and bigger than 1")
            }
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidKey => {
                write!(f, "`key` must be a positive integer and bigger than 1")
            }
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidKey,
}

/* #[derive(Debug)]
pub struct KeyError;

impl std::fmt::Display for KeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: Key must be bigger than 1")
    }
}

impl std::error::Error for KeyError {} */
