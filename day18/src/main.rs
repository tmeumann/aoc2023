use ascii::{AsAsciiStr, AsciiChar};
use std::fmt::{Display, Formatter, Write};
use std::fs::read_to_string;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => f.write_char('U'),
            Direction::Down => f.write_char('D'),
            Direction::Left => f.write_char('L'),
            Direction::Right => f.write_char('R'),
        }
    }
}

impl TryFrom<AsciiChar> for Direction {
    type Error = ();

    fn try_from(value: AsciiChar) -> Result<Self, Self::Error> {
        match value {
            AsciiChar::_0 => Ok(Direction::Right),
            AsciiChar::_1 => Ok(Direction::Down),
            AsciiChar::_2 => Ok(Direction::Left),
            AsciiChar::_3 => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i64,
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let sections: Vec<&str> = value.split_whitespace().collect();

        let hex_str = *sections
            .last()
            .ok_or("Failed to get final space-separated section of input")?;

        let direction = hex_str
            .get_ascii(7)
            .ok_or("Failed to get 7th ascii character")?
            .try_into()
            .map_err(|_| "Failed to convert to direction")?;

        let distance = i64::from_str_radix(
            hex_str
                .slice_ascii(2..hex_str.len() - 2)
                .map_err(|_| "Failed to slice hex number...")?
                .as_str(),
            16,
        )
        .map_err(|_| "Failed to parse number")?;

        Ok(Self {
            direction,
            distance,
        })
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.direction, self.distance)
    }
}

type Coordinate = (i64, i64);

trait Euclidean {
    fn up(&self, distance: i64) -> Self;
    fn down(&self, distance: i64) -> Self;
    fn right(&self, distance: i64) -> Self;
    fn left(&self, distance: i64) -> Self;
}

impl Euclidean for Coordinate {
    fn up(&self, distance: i64) -> Self {
        (self.0, self.1 + distance)
    }

    fn down(&self, distance: i64) -> Self {
        (self.0, self.1 - distance)
    }

    fn right(&self, distance: i64) -> Self {
        (self.0 + distance, self.1)
    }

    fn left(&self, distance: i64) -> Self {
        (self.0 - distance, self.1)
    }
}

fn build_vertices(instructions: &[Instruction]) -> Result<Vec<Coordinate>, ()> {
    let mut coordinates: Vec<Coordinate> = Vec::with_capacity(instructions.len() + 1);

    let mut coordinate: Coordinate = (0, 0);

    for instruction in instructions {
        match instruction.direction {
            Direction::Up => {
                coordinate = coordinate.up(instruction.distance);
            }
            Direction::Down => {
                coordinate = coordinate.down(instruction.distance);
            }
            Direction::Left => {
                coordinate = coordinate.left(instruction.distance);
            }
            Direction::Right => {
                coordinate = coordinate.right(instruction.distance);
            }
        }

        coordinates.push(coordinate);
    }

    Ok(coordinates)
}

fn calculate_area(vertices: &[Coordinate]) -> i64 {
    // shoelace formula -- https://en.wikipedia.org/wiki/Shoelace_formula
    vertices
        .windows(2)
        .map(|edge| edge[0].0 * edge[1].1 - edge[0].1 * edge[1].0)
        .sum::<i64>()
        / 2
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let instructions: Vec<Instruction> = input.lines().flat_map(Instruction::try_from).collect();

    let vertices = build_vertices(&instructions).unwrap();

    let area = calculate_area(&vertices).abs();
    let border: i64 = instructions.iter().map(|i| i.distance.abs()).sum::<i64>();

    // pick's theorem, rearranged to find (b+i) -- https://en.wikipedia.org/wiki/Pick's_theorem
    let total_cubic_metres = area + border / 2 + 1;

    println!("{total_cubic_metres}")
}
