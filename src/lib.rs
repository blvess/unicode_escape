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
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let input = "Hello\\nWorld";
/// let decoded = decode(input).unwrap();
/// assert_eq!(decoded, "Hello\nWorld");
/// ```
///
/// Handling 8-bit escape sequences:
///
/// ```rust
/// let input = "This is a\\x02test";
/// let decoded = decode(input).unwrap();
/// assert_eq!(decoded, "This is a\u{2}test");
/// ```
///
/// Handling Unicode escape sequences:
///
/// ```rust
/// let input = "Unicode: \\u{1F600}";
/// let decoded = decode(input).unwrap();
/// assert_eq!(decoded, "Unicode: 😀");
/// ```
///
/// # Errors
///
/// This function will return an error of type `DecodeError` if an invalid escape sequence is encountered.
///
/// ```rust
/// let input = "Invalid escape: \\z";
/// assert!(decode(input).is_err());
/// ```
///
/// See the `DecodeError` enum for more details on possible error variants.
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
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let mut chars = "02".chars();
/// let decoded = escape_hex(&mut chars).unwrap();
/// assert_eq!(decoded, '\u{2}');
/// ```
///
/// Handling invalid escape sequence:
///
/// ```rust
/// let mut chars = "zz".chars();
/// assert!(escape_hex(&mut chars).is_err());
/// ```
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
/// # Examples
///
/// Basic usage:
///
/// ```rust
/// let mut chars = "{1F600}".chars().peekable();
/// let decoded = decode_unicode(&mut chars).unwrap();
/// assert_eq!(decoded, '😀');
/// ```
///
/// Handling invalid escape sequence:
///
/// ```rust
/// let mut chars = "{zz}".chars().peekable();
/// assert!(decode_unicode(&mut chars).is_err());
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_escape() {
        let mut cases = Vec::new();
        cases.push((r"\t", "\t"));
        cases.push((r"\t\r\n", "\t\r\n"));
        cases.push((r"\t\r\n Hello \0", "\t\r\n Hello \0"));
        cases.push((r"\\", "\\"));
        cases.push((r#"\""#, "\""));
        cases.push((r#"\'"#, "\'"));
        cases.push((r"\0", "\0"));

        for case in cases {
            assert_eq!(decode(case.0).unwrap(), case.1)
        }
    }

    #[test]
    fn test_weight_string() {
        let expected = "\x02 65480 LGM\r\n";
        let case = r"\x02 65480 LGM\r\n";
        assert_eq!(expected, decode(case).unwrap());
    }

    #[test]
    fn test_invalid_escape() {
        let case = r"\x02 \65480 LGM\r\n";
        assert!(decode(case).is_err());
    }

    #[test]
    fn test_unicode_sequence() {
        let expected = "↵";
        let case = r"\u{21B5}";
        assert_eq!(expected, decode(case).unwrap());
        let case = r"\u21B5}";
        assert!(decode(case).is_err());
        let case = r"\u{21B5";
        assert!(decode(case).is_err());
    }

    #[test]
    fn test_unicode_codepoints() {
        let valid_cases = vec!["\\u{1F600}", "\\u{10FFFF}"];
        for case in valid_cases {
            assert!(decode(case).is_ok());
        }

        let invalid_cases = vec!["\\u{110000}", "\\u{FFFFFF}"];
        for case in invalid_cases {
            assert!(decode(case).is_err());
        }
    }
}
