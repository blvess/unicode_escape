/// Defines error types and implementations for decoding escape sequences.
///
/// This module contains the `DecodeError` enum and its associated implementations for displaying
/// and handling decoding errors.
use std::error::Error;
use std::fmt;

/// Represents the different types of errors that can occur during decoding.
#[derive(Debug)]
pub enum DecodeError {
    /// Indicates an invalid escape sequence was encountered.
    InvalidEscape,
    /// Indicates an invalid hexadecimal character was encountered.
    InvalidHexChar,
    /// Indicates an invalid Unicode escape sequence was encountered.
    InvalidUnicode,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for DecodeError {}
