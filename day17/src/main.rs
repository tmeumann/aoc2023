use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs::read_to_string;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn left(&self) -> Direction {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    pub fn right(&self) -> Direction {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Position {
    row: usize,
    column: usize,
    facing: Direction,
    remaining: u8,
}

impl Position {
    pub fn turn_left(&self) -> Self {
        Position {
            row: self.row,
            column: self.column,
            facing: self.facing.left(),
            remaining: 3,
        }
    }

    pub fn turn_right(&self) -> Self {
        Position {
            row: self.row,
            column: self.column,
            facing: self.facing.right(),
            remaining: 3,
        }
    }

    pub fn move_forwards(&self) -> Option<Self> {
        if self.remaining < 1 {
            return None;
        }

        match self.facing {
            Direction::North => self.north(),
            Direction::South => self.south(),
            Direction::East => self.east(),
            Direction::West => self.west(),
        }
    }

    fn north(&self) -> Option<Self> {
        if self.row > 0 {
            Some(Position {
                row: self.row - 1,
                column: self.column,
                facing: self.facing,
                remaining: self.remaining - 1,
            })
        } else {
            None
        }
    }

    fn south(&self) -> Option<Self> {
        Some(Position {
            row: self.row + 1,
            column: self.column,
            facing: self.facing,
            remaining: self.remaining - 1,
        })
    }

    fn east(&self) -> Option<Self> {
        Some(Position {
            row: self.row,
            column: self.column + 1,
            facing: self.facing,
            remaining: self.remaining - 1,
        })
    }

    fn west(&self) -> Option<Self> {
        if self.column > 0 {
            Some(Position {
                row: self.row,
                column: self.column - 1,
                facing: self.facing,
                remaining: self.remaining - 1,
            })
        } else {
            None
        }
    }
}

trait Tiled<T> {
    fn get_tile(&self, position: &Position) -> Option<&T>;
}

impl<T> Tiled<T> for Vec<Vec<T>> {
    fn get_tile(&self, position: &Position) -> Option<&T> {
        self.get(position.row)
            .and_then(|row| row.get(position.column))
    }
}

struct Explorer {
    map: Vec<Vec<u32>>,
    visited: HashSet<Position>,
    queue: BinaryHeap<Reverse<(u32, Position)>>,
    target_row: usize,
    target_column: usize,
}

impl Explorer {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().flat_map(|c| c.to_digit(10)).collect())
            .collect();

        let number_of_rows = map.len();
        let number_of_columns = map.first().unwrap().len();

        let visited: HashSet<Position> =
            HashSet::with_capacity(number_of_columns * number_of_rows * 3 * 4);

        let queue = BinaryHeap::with_capacity(number_of_columns * number_of_rows * 3 * 4);

        Self {
            map,
            visited,
            queue,
            target_row: number_of_rows - 1,
            target_column: number_of_columns - 1,
        }
    }

    fn search(&mut self, starting_position: Position) -> Result<u32, ()> {
        self.queue.push(Reverse((0, starting_position)));
        loop {
            let Reverse((distance, position)) = self.queue.pop().ok_or(())?;

            if self.visited.contains(&position) {
                continue;
            }
            self.visited.insert(position);

            if position.row == self.target_row && position.column == self.target_column {
                return Ok(distance);
            }

            for pos in [
                position.move_forwards(),
                position.turn_left().move_forwards(),
                position.turn_right().move_forwards(),
            ]
            .into_iter()
            .flatten()
            {
                if let Some(tile) = self.map.get_tile(&pos) {
                    self.queue.push(Reverse((distance + tile, pos)))
                }
            }
        }
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut explorer = Explorer::new(&input);

    let result = explorer
        .search(Position {
            row: 0,
            column: 0,
            facing: Direction::North,
            remaining: 3,
        })
        .unwrap();

    println!("{result:?}")
}
