use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to open input file.");
    let reader = BufReader::new(file);

    let sum: u64 = reader
        .lines()
        .map_while(|l| l.ok())
        .filter_map(get_number)
        .sum();

    println!("{}", sum);
}

fn get_number(input: String) -> Option<u64> {
    let digits: Vec<char> = get_digits(input);

    let number_string: String = digits.into_iter().collect();

    number_string.parse().ok()
}

fn get_digits(input: String) -> Vec<char> {
    vec![get_first_digit(&input), get_last_digit(&input)]
        .into_iter()
        .flatten()
        .collect()
}

fn get_first_digit(input: &str) -> Option<char> {
    for i in 0..input.len() {
        for j in i..input.len() {
            if let Some(sub_str) = input.get(i..j + 1) {
                if let Some(val) = parse_digit(sub_str) {
                    return Some(val);
                }
            }
        }
    }
    None
}

fn get_last_digit(input: &str) -> Option<char> {
    for i in (0..input.len()).rev() {
        for j in i..input.len() {
            if let Some(sub_str) = input.get(i..j + 1) {
                if let Some(val) = parse_digit(sub_str) {
                    return Some(val);
                }
            }
        }
    }
    None
}

fn parse_digit(input: &str) -> Option<char> {
    if input.is_empty() {
        return None;
    }

    if input.len() == 1 {
        if let Ok(_) = input.parse::<u64>() {
            return input.chars().next();
        }
    }

    match input.chars().take(3).collect::<String>().as_str() {
        "one" => return Some('1'),
        "two" => return Some('2'),
        "six" => return Some('6'),
        _ => (),
    }

    match input.chars().take(4).collect::<String>().as_str() {
        "four" => return Some('4'),
        "five" => return Some('5'),
        "nine" => return Some('9'),
        _ => (),
    }

    match input.chars().take(5).collect::<String>().as_str() {
        "three" => Some('3'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        _ => None,
    }
}
