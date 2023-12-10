use crate::part1::solve_part_1;
use std::fs::read_to_string;

mod part1;

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input file");

    let solution = solve_part_1(input);

    println!("{solution}");
}
