use unicode_escape::decode;

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
