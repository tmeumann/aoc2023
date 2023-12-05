use std::cmp::min;
use std::fs::read_to_string;
use std::io::BufRead;

#[derive(Eq, PartialEq, Hash)]
struct Game {
    numbers: Vec<u64>,
    winning: Vec<u64>,
}

fn calculate_score(game: &Game) -> u64 {
    let mut total = 0;

    for number in &game.numbers {
        if game.winning.contains(number) {
            total += 1;
        }
    }

    total
}

fn read_game(input: &str) -> Game {
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
    let input = read_to_string("input.txt").expect("Failed to open input file.");

    let games: Vec<Game> = input.lines().map(read_game).collect();
    let num_matching_records: Vec<u64> = games.iter().map(calculate_score).collect();
    let mut card_counts: Vec<u64> = games.iter().map(|_| 1).collect();

    for i in 0..num_matching_records.len() {
        let num_of_this_card = card_counts[i];
        let num_matching = num_matching_records[i];

        for j in
            min(card_counts.len(), i + 1)..min(card_counts.len(), 1 + i + num_matching as usize)
        {
            card_counts[j] += num_of_this_card;
        }
    }

    let total_cards: u64 = card_counts.iter().sum();

    println!("{}", total_cards);
}
