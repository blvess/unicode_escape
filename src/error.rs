use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum DecodeError {
    InvalidEscape,
    InvalidHexChar,
    InvalidUnicode,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DecodeError {}
