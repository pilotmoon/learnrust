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
    let mut first_digit = None;
    let mut last_digit = None;
    for byte in line.bytes() {
        if byte >= b'0' && byte <= b'9' {
            if first_digit.is_none() {
                first_digit = Some(byte - b'0');
            }
            last_digit = Some(byte - b'0');
        }
    }

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => Ok(first * 10 + last),
        _ => Err(format!("Did not find two digits in line: {}", line)),
    }
}
