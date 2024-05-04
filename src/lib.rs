#[derive(Debug)]
pub enum Error {
    InvalidEscape,
    InvalidHexChar,
    InvalidUnicode,
}
pub type Result<T> = std::result::Result<T, Error>;

pub fn decode(input: &str) -> Result<String> {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                // Simple excape sequences
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('0') => result.push('\0'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('\'') => result.push('\''),
                // 8 bit excape sequences
                Some('x') => {
                    let mut hex_chars = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_hexdigit() {
                            hex_chars.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if let Ok(value) = u8::from_str_radix(&hex_chars, 16) {
                        result.push(value as char);
                    } else {
                        return Err(Error::InvalidHexChar);
                    }
                }
                // TODO: unicode escape sequences
                _ => return Err(Error::InvalidEscape),
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
        let case = r"\x02 \x65480 LGM\r\n";
        assert!(decode(case).is_err());
    }

    // #[test]
    // fn test_unicode_sequence() {
    //     let expected = "↵";
    //     let case = r"\u{21B5}";
    //     assert_eq!(expected, decode(case).unwrap());
    // }
}
