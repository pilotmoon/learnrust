use regex::Regex;
use std::io;

#[macro_use]
extern crate lazy_static;

fn main() {
    println!("Started");
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
    lazy_static! {
        static ref RE: Regex =
            Regex::new("[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    }

    // find first and last regular match
    let mut iter = RE.find_iter(line);
    let first_match = iter.next();
    let last_match = iter.last().or(first_match);

    // find overlapping final match, if any
    let last_match = last_match
        .and_then(|m| RE.find(&line[(m.start() + 1)..]))
        .or(last_match);

    // map the end matches to their digits
    let digits = (
        first_match.and_then(|m| decode_digit(m.as_str())),
        last_match.and_then(|m| decode_digit(m.as_str())),
    );

    // combine digits
    match digits {
        (Some(first), Some(last)) => Ok(first * 10 + last),
        _ => Err(format!("No digits in line: {}", line)),
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
