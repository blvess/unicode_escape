use std::error::Error;
use std::fmt;
use std::u32;

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

pub fn decode(input: &str) -> Result<String, impl Error> {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                // Simple excape sequences
                Some('t') => result.push('\t'),
                Some('n') => result.push('\n'),
                Some('r') => result.push('\r'),
                Some('0') => result.push('\0'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                // 8 bit excape sequences \x02
                Some('x') => {
                    let mut hex_chars = String::new();
                    for _ in 0..2 {
                        if let Some(c) = chars.next() {
                            hex_chars.push(c);
                        }
                    }
                    match u8::from_str_radix(&hex_chars, 16) {
                        Ok(value) => result.push(char::from(value)),
                        Err(_) => return Err(DecodeError::InvalidHexChar),
                    };
                }
                // unicde escape /u{1A2B}
                Some('u') => {
                    match chars.next() {
                        Some('{') => '{',
                        _ => return Err(DecodeError::InvalidUnicode),
                    };
                    let mut hex_chars = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_hexdigit() {
                            hex_chars.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if let Ok(value) = u32::from_str_radix(&hex_chars, 16) {
                        if let Some(c) = char::from_u32(value) {
                            result.push(c);
                        } else {
                            return Err(DecodeError::InvalidUnicode);
                        }
                    } else {
                        return Err(DecodeError::InvalidUnicode);
                    }
                    match chars.next() {
                        Some('}') => '}',
                        _ => return Err(DecodeError::InvalidUnicode),
                    };
                }
                _ => return Err(DecodeError::InvalidEscape),
            }
        } else {
            result.push(c);
        }
    }

    Ok(result)
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
}
