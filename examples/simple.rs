use unicode_escape::decode;

fn main() {
    let input = r"\r\n\tHello\u{21B5}";
    let decoded = decode(input).unwrap();
    println!("Decoded string: {}", decoded);
}
