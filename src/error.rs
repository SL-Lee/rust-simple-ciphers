use std::{error, fmt};

#[derive(Debug)]
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidCaesarCipherKey => {
                write!(f, "<key> must be a positive integer between 1 and 25")
            }
            ErrorKind::InvalidColumnarTranspositionCipherKey => {
                write!(
                    f,
                    "<key> must be a string of integers of range 1 to n, where \
                     n is the length of the key itself, in an arbitrary order"
                )
            }
            ErrorKind::InvalidMonoAlphabeticCipherKey => {
                write!(f, "<key> must contain all 26 unique alphabets")
            }
            ErrorKind::InvalidRailFenceCipherKey => {
                write!(f, "<key> must be a positive integer and bigger than 1")
            }
            ErrorKind::InvalidVernamCipherKey => {
                write!(f, "<key> must have the same length as the input")
            }
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidCaesarCipherKey,
    InvalidColumnarTranspositionCipherKey,
    InvalidMonoAlphabeticCipherKey,
    InvalidRailFenceCipherKey,
    InvalidVernamCipherKey,
}
