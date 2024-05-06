//! # Escape Sequence Decoder
//!
//! This crate provides a module for decoding strings with escape sequences. It handles simple escape sequences (e.g., '\t', '\n'), 8-bit escape sequences (e.g., '\x02'), and Unicode escape sequences (e.g., '\u{1A2B}').
//!
//! The module exports a single function, `decode`, which takes a string as input and returns a `Result` containing the decoded string or an error of type `DecodeError`. The function handles invalid escape sequences gracefully, returning an error if an invalid sequence is encountered.
//!
//! The module also provides a set of unit tests to ensure the correctness of the decoding functionality.
use std::iter::Peekable;
use std::u32;

pub mod error;
pub use error::DecodeError;

/// Decodes a string with escape sequences.
///
/// This function interprets and converts escape sequences in the input string into their corresponding characters.
/// It handles simple escape sequences (e.g., '\t', '\n'), 8-bit escape sequences (e.g., '\x02'),
/// and Unicode escape sequences (e.g., '\u{1A2B}').
///
/// # Parameters
///
/// * &str: A string slice or raw string slice
///
/// # Returns
///
/// A `Result` containing a literal string or an error if the escape sequence is invalid.
pub fn decode(input: &str) -> Result<String, DecodeError> {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                // Simple excape sequences ex: \n = newline
                Some('t') => result.push('\t'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('0') => result.push('\0'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                // 8 bit excape sequences ex: \x02 = <STX>
                Some('x') => result.push(escape_hex(&mut chars)?),
                // unicode escape /u{1A2B} = ↵
                Some('u') => result.push(decode_unicode(&mut chars)?),
                _ => return Err(DecodeError::InvalidEscape),
            }
        } else {
            result.push(c);
        }
    }
    Ok(result)
}

/// Decodes a hexadecimal escape sequence.
///
/// This function takes an iterator of characters representing a hexadecimal escape sequence
/// (e.g., `\x02`) and returns the corresponding character.
///
/// # Parameters
///
/// * `chars`: An iterator of characters representing the hexadecimal escape sequence.
///
/// # Returns
///
/// A `Result` containing the decoded character or an error if the escape sequence is invalid.
///
/// # Errors
///
/// This function will return an error of type `DecodeError::InvalidHexChar` if the escape sequence
/// is not a valid hexadecimal representation of a character.
fn escape_hex(chars: &mut impl Iterator<Item = char>) -> Result<char, DecodeError> {
    let mut hex_chars = String::new();
    for _ in 0..2 {
        if let Some(c) = chars.next() {
            hex_chars.push(c);
        } else {
            return Err(DecodeError::InvalidHexChar);
        }
    }
    match u8::from_str_radix(&hex_chars, 16) {
        Ok(value) => Ok(char::from(value)),
        Err(_) => Err(DecodeError::InvalidHexChar),
    }
}

/// Decodes a Unicode escape sequence.
///
/// This function takes an iterator of characters representing a Unicode escape sequence
/// (e.g., `\u{1F600}`) and returns the corresponding character.
///
/// # Parameters
///
/// * `chars`: An iterator of characters representing the Unicode escape sequence.
///
/// # Returns
///
/// A `Result` containing the decoded character or an error if the escape sequence is invalid.
///
/// # Errors
///
/// This function will return an error of type `DecodeError::InvalidUnicode` if the escape sequence
/// is not a valid Unicode representation of a character or if the Unicode code point is out of range.
fn decode_unicode(chars: &mut Peekable<impl Iterator<Item = char>>) -> Result<char, DecodeError> {
    // Remove the leading '{'
    match chars.next() {
        Some('{') => {}
        _ => return Err(DecodeError::InvalidUnicode),
    };

    // Gather all hex digits in a string
    let mut hex_chars = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_hexdigit() {
            hex_chars.push(c);
            chars.next();
        } else {
            break;
        }
    }

    // Remove the trailing '}'
    match chars.next() {
        Some('}') => {}
        _ => return Err(DecodeError::InvalidUnicode),
    };

    // Convert the stirng to a char
    if let Ok(value) = u32::from_str_radix(&hex_chars, 16) {
        if let Some(c) = char::from_u32(value) {
            Ok(c)
        } else {
            Err(DecodeError::InvalidUnicode)
        }
    } else {
        Err(DecodeError::InvalidUnicode)
    }
}
