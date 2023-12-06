use rayon::prelude::*;
use std::fs::read_to_string;

struct MapEntry {
    source_start: i64,
    destination_start: i64,
    length: i64,
}

impl MapEntry {
    pub fn new(input: &str) -> Self {
        let numbers: Vec<i64> = input.split(' ').flat_map(|s| s.parse::<i64>()).collect();

        let destination_start = numbers[0];
        let source_start = numbers[1];
        let length = numbers[2];

        Self {
            destination_start,
            source_start,
            length,
        }
    }

    pub fn calculate_destination(&self, source: i64) -> Option<i64> {
        let source_max = self.source_start + self.length - 1;

        if source < self.source_start || source > source_max {
            return None;
        }

        let difference = self.destination_start - self.source_start;

        Some(source + difference)
    }
}

struct Mapperer {
    map_entries: Vec<MapEntry>,
}

impl Mapperer {
    pub fn new(input: &str) -> Self {
        let map_entries = input.lines().skip(1).map(MapEntry::new).collect();

        Self { map_entries }
    }

    pub fn calculate_destination(&self, source: i64) -> i64 {
        self.map_entries
            .iter()
            .filter_map(|m| m.calculate_destination(source))
            .next()
            .unwrap_or(source)
    }
}

fn main() {
    let input_string = read_to_string("input.txt").unwrap();
    let input_sections: Vec<&str> = input_string.split("\n\n").collect();

    let seed_input_numbers: Vec<i64> = input_sections
        .first()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .trim()
        .split(' ')
        .flat_map(|s| s.parse::<i64>())
        .collect();

    let mapperers: Vec<Mapperer> = input_sections
        .into_iter()
        .skip(1)
        .map(Mapperer::new)
        .collect();

    let result = seed_input_numbers
        .chunks_exact(2)
        .par_bridge()
        .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .map(|n| {
            mapperers
                .iter()
                .fold(n, |acc, m| m.calculate_destination(acc))
        })
        .min()
        .unwrap();

    println!("{result}");
}
