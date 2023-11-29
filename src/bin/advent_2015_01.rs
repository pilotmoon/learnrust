use std::io::{self, Read};

fn main() {
    let mut floor = 0;
    let mut entered_basement = false;
    for (pos, byte) in io::stdin().bytes().enumerate() {
        match byte {
            Ok(b'(') => floor += 1,
            Ok(b')') => floor -= 1,
            Ok(x) => println!("Unexpected byte: {}", x),
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        }
        if floor == -1 && !entered_basement {
            entered_basement = true;
            println!("Santa first enters the basement at position {}", pos + 1);
        }
    }
    println!("Santa finishes on floor {}", floor);
}
