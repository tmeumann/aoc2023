use std::fs::read_to_string;

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .flat_map(|s| s.parse::<i64>())
        .collect()
}

fn calculate_next_value(numbers: Vec<i64>) -> i64 {
    if numbers.iter().all(|&n| n == 0) {
        return 0;
    }

    calculate_next_value(numbers.windows(2).map(|w| w[1] - w[0]).collect())
        + numbers.last().expect("Empty numbers")
}

fn calculate_previous_value(numbers: Vec<i64>) -> i64 {
    if numbers.iter().all(|&n| n == 0) {
        return 0;
    }

    numbers.first().expect("Empty numbers")
        - calculate_previous_value(numbers.windows(2).map(|w| w[1] - w[0]).collect())
}

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input file");

    let result: i64 = input
        .lines()
        .map(parse_line)
        .map(calculate_previous_value)
        .sum();

    println!("{result}");
}
