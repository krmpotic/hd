#[derive(Debug, PartialEq)]
enum Ascii {
    LowChar,
    BigChar,
    Symbol,
    Digit,
    Whitespace,
}

pub fn to_canonical(c: u8) -> char {
    match is_ascii(c) {
        Some(Ascii::LowChar) => c as char,
        Some(Ascii::BigChar) => c as char,
        Some(Ascii::Digit) => c as char,
        Some(Ascii::Whitespace) => ' ',
        Some(Ascii::Symbol) => c as char,
        None => '?',
    }
}

fn is_ascii(c: u8) -> Option<Ascii> {
    if b'a' <= c && c <= b'z' {
        Some(Ascii::LowChar)
    } else if b'A' <= c && c <= b'Z' {
        Some(Ascii::BigChar)
    } else if b'0' <= c && c <= b'9' {
        Some(Ascii::Digit)
    } else if c == b'\n' || c == b' ' || c == b'\t' {
        Some(Ascii::Whitespace)
    } else if c == b'['
        || c == b']'
        || c == b'('
        || c == b')'
        || c == b'{'
        || c == b'}'
        || c == b'='
        || c == b'"'
        || c == b'\''
    {
        Some(Ascii::Symbol)
    } else {
        None
    }
}

#[test]
fn test_is_ascii() {
    assert_eq!(is_ascii(0xff), None);
    assert_eq!(is_ascii(b'c'), Some(Ascii::LowChar));
    assert_eq!(is_ascii(b'C'), Some(Ascii::BigChar));
    assert_eq!(is_ascii(b'\n'), Some(Ascii::Whitespace));
}
