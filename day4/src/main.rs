use std::fs::File;
use std::io::{BufRead, BufReader};

struct Game {
    numbers: Vec<u64>,
    winning: Vec<u64>,
}

fn calculate_score(game: Game) -> u64 {
    let mut total = 0;

    for number in &game.numbers {
        if game.winning.contains(number) {
            if total == 0 {
                total = 1;
            } else {
                total *= 2;
            }
        }
    }

    total
}

fn read_game(input: String) -> Game {
    let after_colon = input.split(':').last().unwrap();
    let sub_strings: Vec<&str> = after_colon.split('|').map(|s| s.trim()).collect();

    let winning: Vec<u64> = sub_strings
        .first()
        .unwrap()
        .split(' ')
        .flat_map(|s| s.parse::<u64>())
        .collect();

    let numbers = sub_strings
        .last()
        .unwrap()
        .split(' ')
        .flat_map(|s| s.parse::<u64>())
        .collect();

    Game { numbers, winning }
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open input file.");
    let reader = BufReader::new(file);

    let total: u64 = reader
        .lines()
        .map_while(|l| l.ok())
        .map(read_game)
        .map(calculate_score)
        .sum();

    println!("{}", total);
}
