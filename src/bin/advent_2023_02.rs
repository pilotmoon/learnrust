// 2023 Day 2 "Cube Conundrum" https://adventofcode.com/2023/day/2
use std::cmp;
use std::io;

fn main() {
    println!("Started");
    let mut sum: usize = 0;
    let mut sum_power: usize = 0;
    for (i, line) in io::stdin().lines().enumerate() {
        match line {
            Ok(line) => match parse_game(&line) {
                Ok(game) => {
                    let (possible, power) = analyse(game, (12, 13, 14));
                    if possible {
                        sum += i + 1;
                    }
                    sum_power += power;
                }
                Err(e) => {
                    eprintln!("Error parsing game: {}", e);
                }
            },
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                return;
            }
        }
    }
    println!(
        "The sum of possible game id's is {}, sum of powers is {}",
        sum, sum_power
    );
}

fn analyse(game: Vec<(usize, usize, usize)>, stock: (usize, usize, usize)) -> (bool, usize) {
    let mut seen = (0, 0, 0);
    for subset in game.iter() {
        seen.0 = cmp::max(seen.0, subset.0);
        seen.1 = cmp::max(seen.1, subset.1);
        seen.2 = cmp::max(seen.2, subset.2);
    }
    (
        seen.0 <= stock.0 && seen.1 <= stock.1 && seen.2 <= stock.2,
        seen.0 * seen.1 * seen.2,
    )
}

fn parse_game(line: &str) -> Result<Vec<(usize, usize, usize)>, String> {
    match line.split_once(':') {
        Some((_, game_str)) => {
            let mut game = Vec::new();
            for round_str in game_str.split(';').map(str::trim) {
                let mut round = (0, 0, 0);
                for subset_str in round_str.split(',').map(str::trim) {
                    match subset_str.split_once(' ') {
                        Some((num, col)) => {
                            let num: Result<usize, _> = num.parse();
                            if let Ok(num) = num {
                                match col {
                                    "red" => round.0 += num,
                                    "green" => round.1 += num,
                                    "blue" => round.2 += num,
                                    _ => todo!("unexpected color"),
                                }
                            }
                        }
                        None => return Err(format!("Error parsing subset {}", subset_str)),
                    }
                }
                game.push(round);
            }
            return Ok(game);
        }
        None => return Err(format!("Error parsing line: {}", line)),
    }
}
