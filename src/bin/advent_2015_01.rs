use std::io::{self, Read};

fn main() {
    let mut floor = 0;
    for (pos, byte) in io::stdin().bytes().enumerate() {
        match byte {
            Ok(b'(') => floor += 1,
            Ok(b')') => floor -= 1,
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        }
        if floor == -1 {
            println!("Santa entered the basement at position {}", pos + 1);
        }
    }
    println!("Santa finishes on floor {}", floor);
}
