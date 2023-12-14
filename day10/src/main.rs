use crate::part2::solve_part_2;
use std::fs::read_to_string;

mod part1;
mod part2;

fn main() {
    let input = read_to_string("input.txt").expect("Failed to read input file");

    let solution = solve_part_2(input);

    println!("{solution}");
}
