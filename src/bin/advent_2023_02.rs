// 2023 Day 2 "Cube Conundrum" https://adventofcode.com/2023/day/2
use std::cmp;
use std::io;

fn main() {
    println!("Started");
    let stock = ColorCounts {
        r: 12,
        g: 13,
        b: 14,
    };
    let mut sum: usize = 0;
    let mut sum_power: usize = 0;
    for (i, line) in io::stdin().lines().enumerate() {
        match line {
            Ok(line) => match parse_game(&line) {
                Ok(game) => {
                    let (possible, power) = analyse(&game, &stock);
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

struct ColorCounts {
    r: usize,
    g: usize,
    b: usize,
}

fn analyse(game: &Vec<ColorCounts>, stock: &ColorCounts) -> (bool, usize) {
    let seen = game
        .iter()
        .fold(ColorCounts { r: 0, g: 0, b: 0 }, |acc, x| ColorCounts {
            r: cmp::max(acc.r, x.r),
            g: cmp::max(acc.g, x.g),
            b: cmp::max(acc.b, x.b),
        });
    (
        seen.r <= stock.r && seen.g <= stock.g && seen.g <= stock.b,
        seen.r * seen.g * seen.b,
    )
}

fn parse_game(line: &str) -> Result<Vec<ColorCounts>, String> {
    match line.split_once(':') {
        Some((_, game_str)) => {
            let mut game = Vec::new();
            for round_str in game_str.split(';').map(str::trim) {
                let mut round = ColorCounts { r: 0, g: 0, b: 0 };
                for subset_str in round_str.split(',').map(str::trim) {
                    match subset_str.split_once(' ') {
                        Some((num, col)) => {
                            let num: Result<usize, _> = num.parse();
                            if let Ok(num) = num {
                                match col {
                                    "red" => round.r += num,
                                    "green" => round.g += num,
                                    "blue" => round.b += num,
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
