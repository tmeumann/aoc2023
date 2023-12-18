use ascii::{AsciiChar, AsciiStr, IntoAsciiString};
use std::fs::read_to_string;

fn hash(s: &AsciiStr) -> u64 {
    s.chars().fold(0, |acc, character| {
        (acc + character.as_byte() as u64) * 17 % 256
    })
}

fn main() {
    let input = read_to_string("input.txt")
        .unwrap()
        .into_ascii_string()
        .unwrap();

    let result: u64 = input.split(AsciiChar::Comma).map(hash).sum();

    println!("{result}")
}
