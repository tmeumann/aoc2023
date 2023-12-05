use std::cmp::max;
use std::fs::{read_to_string, File};
use std::io::{BufRead, BufReader, Read, Seek};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn is_within(&self, rectangle: &Rectangle) -> bool {
        rectangle.contains(self)
    }
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn contains(&self, point: &Point) -> bool {
        self.top_left.x <= point.x
            && point.x <= self.bottom_right.x
            && self.top_left.y <= point.y
            && point.y <= self.bottom_right.y
    }
}

#[derive(Debug)]
struct Number {
    value: u64,
    location: Point, // left-most digit
    length: usize,
}

impl Number {
    pub fn is_adjacent_to_symbol(&self, symbols: &[Vec<bool>]) -> bool {
        let left = self.location.x.saturating_sub(1);
        let right = self.location.x + self.length;
        let top = self.location.y.saturating_sub(1);
        let bottom = self.location.y + 1;

        for x in left..=right {
            for y in top..=bottom {
                if let Some(row) = symbols.get(y) {
                    if let Some(symbol) = row.get(x) {
                        if *symbol {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

fn build_symbol_arrays(input: &str) -> Vec<Vec<bool>> {
    let mut symbols: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        let mut symbols_on_line = Vec::new();

        for c in line.chars() {
            symbols_on_line.push(is_symbol(c));
        }

        symbols.push(symbols_on_line);
    }

    symbols
}

fn build_number_array(input: &str) -> Vec<Number> {
    let mut numbers = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut last_char_was_digit = false;

        for x in 0..line.len() {
            if let Some(remainder) = line.get(x..line.len()) {
                if let Some(c) = remainder.chars().next() {
                    if !c.is_ascii_digit() {
                        last_char_was_digit = false;
                        continue;
                    }

                    if !last_char_was_digit {
                        let the_number_string: String = remainder
                            .chars()
                            .take_while(|c| c.is_ascii_digit())
                            .collect();

                        if let Ok(value) = the_number_string.parse::<u64>() {
                            numbers.push(Number {
                                value,
                                location: Point { x, y },
                                length: the_number_string.len(),
                            })
                        }
                    }

                    last_char_was_digit = true;
                }
            }
        }
    }

    numbers
}

fn main() {
    let input: String = read_to_string("input.txt").expect("Failed to read input");

    let symbols = build_symbol_arrays(&input);

    let numbers = build_number_array(&input);

    let sum: u64 = numbers
        .iter()
        .filter(|n| n.is_adjacent_to_symbol(&symbols))
        .map(|n| n.value)
        .sum();

    println!("{}", sum);
}
