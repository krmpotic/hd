#[derive(Debug, PartialEq)]
enum Special {
    Null,
    Delete,
    Other,
}

#[derive(Debug, PartialEq)]
enum Ascii {
    Special(Special),
    LowChar(char),
    BigChar(char),
    Symbol(char),
    Digit(u8),
    Whitespace(char),
}

pub fn to_canonical(b: u8) -> char {
    match is_ascii(b) {
        Some(Ascii::LowChar(c)) => c,
        Some(Ascii::BigChar(c)) => c,
        Some(Ascii::Digit(_)) => b as char,
        Some(Ascii::Whitespace(_)) => ' ',
        Some(Ascii::Symbol(c)) => c,
        Some(Ascii::Special(_)) => '#',
        None => '?',
    }
}

fn is_ascii(c: u8) -> Option<Ascii> {
    if b'a' <= c && c <= b'z' {
        Some(Ascii::LowChar(c as char))
    } else if b'A' <= c && c <= b'Z' {
        Some(Ascii::BigChar(c as char))
    } else if b'0' <= c && c <= b'9' {
        Some(Ascii::Digit(c - b'0'))
    } else if c == b'\n' || c == b' ' || c == b'\t' || c == b'\t' || c == 0x0b {
        Some(Ascii::Whitespace(c as char))
    } else if (c >= 0x21 && c <= 0x2f)
        || (c >= 0x3a && c <= 0x40)
        || (c >= 0x5b && c <= 0x60)
        || (c >= 0x7b && c <= 0x7e)
    {
        Some(Ascii::Symbol(c as char))
    } else if c <= 0x20 || c == 0x7f {
        match c {
            0x00 => Some(Ascii::Special(Special::Null)),
            0x7f => Some(Ascii::Special(Special::Delete)),
            _ => Some(Ascii::Special(Special::Other)),
        }
    } else {
        None
    }
}

#[test]
fn test_is_ascii() {
    assert_eq!(is_ascii(0xff), None);
    assert_eq!(is_ascii(b'c'), Some(Ascii::LowChar('c')));
    assert_eq!(is_ascii(b'C'), Some(Ascii::BigChar('C')));
    assert_eq!(is_ascii(b'\n'), Some(Ascii::Whitespace('\n')));
    assert_eq!(is_ascii(b'3'), Some(Ascii::Digit(3)));
    assert_eq!(is_ascii(0x7f), Some(Ascii::Special(Special::Delete)));
    assert_eq!(is_ascii(0x00), Some(Ascii::Special(Special::Null)));
}
