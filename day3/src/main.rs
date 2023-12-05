use std::fs::read_to_string;
use std::io::{BufRead, Read};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Number {
    value: u64,
    location: Point, // left-most digit
    length: usize,
}

impl Number {
    pub fn add_to_nearby_gears(&self, symbols: &mut [Vec<Option<Vec<u64>>>]) {
        let left = self.location.x.saturating_sub(1);
        let right = self.location.x + self.length;
        let top = self.location.y.saturating_sub(1);
        let bottom = self.location.y + 1;

        for x in left..=right {
            for y in top..=bottom {
                if let Some(row) = symbols.get_mut(y) {
                    if let Some(Some(symbol)) = row.get_mut(x) {
                        symbol.push(self.value);
                    }
                }
            }
        }
    }
}

fn is_gear(c: char) -> bool {
    c == '*'
}

fn build_gear_arrays(input: &str) -> Vec<Vec<Option<Vec<u64>>>> {
    let mut symbols: Vec<Vec<Option<Vec<u64>>>> = Vec::new();

    for line in input.lines() {
        let mut symbols_on_line = Vec::new();

        for c in line.chars() {
            symbols_on_line.push(if is_gear(c) { Some(Vec::new()) } else { None });
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

    let mut gears = build_gear_arrays(&input);

    let numbers = build_number_array(&input);

    for number in numbers {
        number.add_to_nearby_gears(&mut gears);
    }

    let mut total: u64 = 0;

    for row in gears {
        for gear in row.iter().flatten() {
            if gear.len() > 1 {
                total += gear.iter().product::<u64>();
            }
        }
    }

    println!("{}", total);
}
