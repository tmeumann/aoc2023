use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Sample {
    red: u64,
    green: u64,
    blue: u64,
}

impl Sample {
    pub fn parse(input: &str) -> Self {
        let mut red: u64 = 0;
        let mut green: u64 = 0;
        let mut blue: u64 = 0;

        let counts: Vec<&str> = input.split(',').map(|s| s.trim()).collect();

        for count in counts {
            let tokens: Vec<&str> = count.split(' ').map(|s| s.trim()).collect();

            let count = tokens.first().unwrap();
            let colour = tokens.last().unwrap();

            match *colour {
                "red" => {
                    if let Some(count) = count.parse::<u64>().ok() {
                        red += count;
                    }
                }
                "green" => {
                    if let Some(count) = count.parse::<u64>().ok() {
                        green += count;
                    }
                }
                "blue" => {
                    if let Some(count) = count.parse::<u64>().ok() {
                        blue += count;
                    }
                }
                _ => (),
            }
        }

        Self { red, green, blue }
    }

    pub fn total_seen(samples: Vec<Self>) -> Self {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for Self { red, green, blue } in samples {
            max_red = max(red, max_red);
            max_green = max(green, max_green);
            max_blue = max(blue, max_blue);
        }

        Self {
            red: max_red,
            green: max_green,
            blue: max_blue,
        }
    }
}

fn calculate_power(input: String) -> u64 {
    let tokens: Vec<&str> = input.split(':').map(|s| s.trim()).collect();

    let samples = tokens.last().unwrap();

    let samples: Vec<Sample> = samples
        .split(';')
        .map(|s| s.trim())
        .map(Sample::parse)
        .collect();

    let Sample { red, green, blue } = Sample::total_seen(samples);

    red * green * blue
}

fn main() {
    let file = File::open("input.txt").expect("Failed to open input file.");
    let reader = BufReader::new(file);

    let sum: u64 = reader
        .lines()
        .map_while(|l| l.ok())
        .map(calculate_power)
        .sum();

    println!("{}", sum);
}
