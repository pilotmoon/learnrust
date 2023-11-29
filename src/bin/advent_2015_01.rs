use std::io::{self, Read};

fn main() {
    let mut floor = 0;
    for byte in io::stdin().bytes() {
        match byte {
            Ok(b'(') => floor += 1,
            Ok(b')') => floor -= 1,
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        }
    }
    println!("Santa finishes on floor {}", floor);
}
