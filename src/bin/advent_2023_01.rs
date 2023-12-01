use regex::Regex;
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

    // find digts in line using sliding window
    // because last digit in "3two3eightjszbfourkxbh5twonepr" is "one"
    let re = Regex::new("^([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    for (i, _) in line.chars().enumerate() {
        if let Some(m) = re.find(&line[i..]) {
            match decode_digit(m.as_str()) {
                Some(d) => digits.push(d),
                None => return Err(format!("Could not decode: {}", m.as_str())),
            }
        }
    }

    match digits[..] {
        [only] => Ok(only * 10 + only),
        [first, .., last] => Ok(first * 10 + last),
        _ => Err(format!("Did not find digits in line: {}", line)),
    }
}

fn decode_digit(digit_str: &str) -> Option<u8> {
    match digit_str {
        "1" | "one" => Some(1),
        "2" | "two" => Some(2),
        "3" | "three" => Some(3),
        "4" | "four" => Some(4),
        "5" | "five" => Some(5),
        "6" | "six" => Some(6),
        "7" | "seven" => Some(7),
        "8" | "eight" => Some(8),
        "9" | "nine" => Some(9),
        _ => None,
    }
}
