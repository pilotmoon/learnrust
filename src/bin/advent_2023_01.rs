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
    let mut first = None;
    let mut last = None;

    // find first digit in the line
    let re = Regex::new(r"[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    if let Some(m) = re.find(line) {
        match decode_digit(m.as_str()) {
            Some(d) => first = Some(d),
            None => return Err(format!("Could not decode: {}", m.as_str())),
        }
    }

    // find last digit in the line
    let rf = Regex::new(r"[1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let rev_line: String = line.chars().rev().collect();
    if let Some(m) = rf.find(&rev_line) {
        let rev_m: String = m.as_str().chars().rev().collect();
        match decode_digit(&rev_m) {
            Some(d) => last = Some(d),
            None => return Err(format!("Could not decode: {}", rev_m.as_str())),
        }
    }

    match (first, last) {
        (Some(first), Some(last)) => Ok(first * 10 + last),
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