use std::fs::File;
use std::io::Read;

fn main() -> std::io::Result<()> {
    let mut file = File::open("Cargo.toml")?;
    prnt(&mut file, true);
    Ok(())
}

fn prnt(file: &mut File, show_ascii: bool) {
    let mut i = 0;
    const SIZE: usize = 16;
    loop {
        let mut buf = [0; SIZE];
        match file.read(&mut buf) {
            Ok(n) => {
                print!("0x{:08x}", i);
                for (i, c) in buf.into_iter().take(n).enumerate() {
                    if i % 8 == 0 {
                        print!(" ");
                    }
                    print!(" {:02x}", c);
                }
                if show_ascii {
                    for _ in 0..SIZE - n {
                        print!("   ")
                    }
                    print!("  |");
                    for c in buf.into_iter().take(n) {
                        print!("{}", to_canonical(c))
                    }
                    print!("|")
                }
                println!();
                i = i + n;
                if n < SIZE {
                    print!("0x{:08x} ", i);
                    return;
                }
            }
            Err(..) => {
                return;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Ascii {
    LowChar,
    BigChar,
    Symbol,
    Digit,
    Whitespace,
}

fn to_canonical(c: u8) -> char {
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
