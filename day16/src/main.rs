mod contraption;
mod tile;

use crate::contraption::Contraption;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut contraption = Contraption::new(&input);

    let result = contraption.calculate_max_energy();

    println!("{result}")
}
