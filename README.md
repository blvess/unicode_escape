# Unicode string escape

A Rust crate for decoding escape sequences in strings.

## Overview

This crate provides a simple way to decode escape sequences in Rust strings. It supports various types of escape sequences, including simple escape sequences (e.g., `\t`, `\n`), hex escape sequences (e.g., `\x02`), and Unicode escape sequences (e.g., `\u{1A2B}`). It also handles invalid escape sequences and provides error handling and reporting.

This crate attempts to replicate the features provided by python's

```python
bytes(<input>, 'ascii').decode('unicode_escape')
```

## Installation

To use this crate, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
unicode_escape = "0.1.0"
```

## Usage

The crate provides a `decode` function that takes a &str with escape sequences as input and returns a decoded String. It also defines a `DecodeError` enum to represent different types of errors that can occur during decoding.

Here's an example of how to use the `decode` function:

```rust
use unicode_escape::decode;

let input = r"\r\n\tHello\u{21B5}";
let decoded = decode(input).unwrap();
println!("Decoded string: {}", decoded);
```

```
Decoded string:
        Hello↵
```

In this example, the input string contains various escape sequences, including tab (`\t`), newline (`\n`), hex escape (`\x02`), and Unicode escape (`\u{21B5}`). The `decode` function will replace these escape sequences with their corresponding characters, and the decoded string will be printed.

## Error Handling

The `decode` function returns a `Result<String, DecodeError>` to indicate success or failure. The `DecodeError` enum includes variants such as `InvalidEscape`, `InvalidHexChar`, and `InvalidUnicode` to provide more context about the error that occurred.

Here's an example of handling errors:

```rust
use unicode_escape::{decode, DecodeError};

let input = r"\t\r\n Hello \xGG\u{ZZZZ}";
match decode(input) {
    Ok(decoded) => println!("Decoded string: {}", decoded),
    Err(error) => println!("Error: {:?}", error),
}
```

In this example, the input string contains invalid hex characters (`\xGG`) and invalid Unicode escape sequences (`\u{ZZZZ}`). The `decode` function will return an `Err` value, and the error variant can be inspected to determine the specific error that occurred.

## License

This project is licensed under the MIT License. For more information, see the [LICENSE](LICENSE.md) file.
