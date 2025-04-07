mod ascii;

use ascii::*;
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
