use std::io::{self, Read};

fn main() {
    let mut floor = 0;
    let mut basement_pos: Option<usize> = None;
    for (pos, byte) in io::stdin().bytes().enumerate() {
        match byte {
            Ok(b'(') => floor += 1,
            Ok(b')') => floor -= 1,
            Ok(_) => continue,
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                return;
            }
        }
        if floor == -1 && basement_pos.is_none() {
            basement_pos = Some(pos);
        }
    }
    if let Some(pos) = basement_pos {
        println!("Santa first entered the basement at position {}", pos + 1);
    } else {
        println!("Santa never entered the basement")
    }
    println!("Santa finished on floor {}", floor);
}
