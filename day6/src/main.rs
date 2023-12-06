use std::fs::read_to_string;

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

    let time_string: String = input_string
        .lines()
        .next()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .collect();
    let time: i64 = time_string.parse().unwrap();

    let distance_string: String = input_string
        .lines()
        .last()
        .unwrap()
        .split(':')
        .last()
        .unwrap()
        .split_whitespace()
        .collect();
    let distance: i64 = distance_string.parse().unwrap();

    let race = Race::new((time, distance));

    let ways_to_beat = race.ways_to_beat();

    println!("{ways_to_beat}");
}
