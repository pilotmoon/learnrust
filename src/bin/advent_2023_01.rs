use std::io;

fn main() {
    let mut sum: u64 = 0;
    for line in io::stdin().lines() {
        match line {
            Ok(line) => match decode_line(&line) {
                Ok(num) => {
                    println!("{} decoded to {}", line, num);
                    sum += num as u64;
                }
                Err(msg) => {
                    eprintln!("Error decoding line: {}", msg);
                    return;
                }
            },
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return;
            }
        }
    }
    println!("The final sum is {}", sum);
}

fn decode_line(line: &str) -> Result<u8, String> {
    let mut digits: Vec<u8> = Vec::new();

    for byte in line.bytes() {
        if byte.is_ascii_digit() {
            digits.push(byte - b'0');
        }
    }

    match digits[..] {
        [only] => Ok(only * 10 + only),
        [first, .., last] => Ok(first * 10 + last),
        _ => Err(format!("Did not find digits in line: {}", line)),
    }
}
