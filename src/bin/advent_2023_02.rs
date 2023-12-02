// 2023 Day 2 "Cube Conundrum" https://adventofcode.com/2023/day/2
use std::cmp;
use std::io;

fn main() {
    println!("Started");
    let mut sum: usize = 0;
    for (i, line) in io::stdin().lines().enumerate() {
        match line {
            Ok(line) => match parse_game(&line) {
                Ok(game) => {
                    if possible(game, (12, 13, 14)) {
                        sum += i + 1;
                    }
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
    println!("The final sum is {}", sum);
}

fn possible(game: Vec<(usize, usize, usize)>, stock: (usize, usize, usize)) -> bool {
    let mut max = (0, 0, 0);
    for subset in game.iter() {
        max.0 = cmp::max(max.0, subset.0);
        max.1 = cmp::max(max.1, subset.1);
        max.2 = cmp::max(max.2, subset.2);
    }
    max.0 <= stock.0 && max.1 <= stock.1 && max.2 <= stock.2
}

fn parse_game(line: &str) -> Result<Vec<(usize, usize, usize)>, String> {
    match line.split_once(':') {
        Some((_, game_str)) => {
            let mut game = Vec::new();
            for round_str in game_str.split(';').map(|s| s.trim()) {
                let mut round = (0, 0, 0);
                for subset_str in round_str.split(',').map(|s| s.trim()) {
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
                        None => return Err("Error parsing subset".to_string()),
                    }
                }
                game.push(round);
            }
            return Ok(game);
        }
        None => return Err("Error parsing game".to_string()),
    }
}
