use std::fs::read_to_string;
use std::iter::zip;

struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    pub fn new((time, distance): (i64, i64)) -> Self {
        Self { time, distance }
    }

    pub fn ways_to_beat(&self) -> i64 {
        // solvable with the quadratic equation instead of search
        let sqrt = ((self.time.pow(2) - 4 * self.distance) as f64).sqrt();

        let short_press = (-self.time as f64 - sqrt) / 2.0;
        let long_press = (-self.time as f64 + sqrt) / 2.0;

        (long_press.ceil() - short_press.floor()) as i64 - 1
    }
}

fn main() {
    let input_string = read_to_string("input.txt").unwrap();

    let times: i64 = zip(
        input_string
            .lines()
            .next()
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .split_whitespace()
            .flat_map(|s| s.parse::<i64>()),
        input_string
            .lines()
            .last()
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .split_whitespace()
            .flat_map(|s| s.parse::<i64>()),
    )
    .map(Race::new)
    .map(|r| r.ways_to_beat())
    .product();

    println!("{times:?}");
}
