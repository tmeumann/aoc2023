use crate::tile::{Orientation, Tile, Tilt};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

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

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Position {
    row: usize,
    column: usize,
    facing: Direction,
}

impl Position {
    pub fn turn_left(&self) -> Self {
        Position {
            row: self.row,
            column: self.column,
            facing: self.facing.left(),
        }
    }

    pub fn turn_right(&self) -> Self {
        Position {
            row: self.row,
            column: self.column,
            facing: self.facing.right(),
        }
    }

    pub fn move_forwards(&self) -> Option<Self> {
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
        })
    }

    fn east(&self) -> Option<Self> {
        Some(Position {
            row: self.row,
            column: self.column + 1,
            facing: self.facing,
        })
    }

    fn west(&self) -> Option<Self> {
        if self.column > 0 {
            Some(Position {
                row: self.row,
                column: self.column - 1,
                facing: self.facing,
            })
        } else {
            None
        }
    }
}

trait Map<T> {
    fn get_tile(&self, position: &Position) -> Option<&T>;
}

impl<T> Map<T> for Vec<Vec<T>> {
    fn get_tile(&self, position: &Position) -> Option<&T> {
        self.get(position.row)
            .map(|row| row.get(position.column))
            .flatten()
    }
}

pub struct Contraption {
    schematic: Vec<Vec<Tile>>,
    seen_beams: HashSet<Position>,
}

impl Contraption {
    pub fn new(input: &str) -> Self {
        let schematic = input
            .lines()
            .map(|row| row.chars().map(Tile::new).collect())
            .collect();

        let beams = HashSet::new();

        Self {
            schematic,
            seen_beams: beams,
        }
    }

    fn track_beam(&mut self, position: Option<Position>) {
        let position = match position {
            Some(p) => p,
            None => {
                return;
            }
        };

        let tile = match self.schematic.get_tile(&position) {
            Some(t) => t,
            None => {
                return;
            }
        };

        if self.seen_beams.contains(&position) {
            return;
        }

        self.seen_beams.insert(position);

        match tile {
            Tile::Empty => self.track_beam(position.move_forwards()),
            Tile::Mirror(Tilt::Right) => match position.facing {
                Direction::North | Direction::South => {
                    self.track_beam(position.turn_right().move_forwards())
                }
                Direction::East | Direction::West => {
                    self.track_beam(position.turn_left().move_forwards())
                }
            },
            Tile::Mirror(Tilt::Left) => match position.facing {
                Direction::North | Direction::South => {
                    self.track_beam(position.turn_left().move_forwards())
                }
                Direction::East | Direction::West => {
                    self.track_beam(position.turn_right().move_forwards())
                }
            },
            Tile::Splitter(Orientation::Vertical) => match position.facing {
                Direction::North | Direction::South => self.track_beam(position.move_forwards()),
                Direction::East | Direction::West => {
                    self.track_beam(position.turn_left().move_forwards());
                    self.track_beam(position.turn_right().move_forwards());
                }
            },
            Tile::Splitter(Orientation::Horizontal) => match position.facing {
                Direction::North | Direction::South => {
                    self.track_beam(position.turn_left().move_forwards());
                    self.track_beam(position.turn_right().move_forwards());
                }
                Direction::East | Direction::West => self.track_beam(position.move_forwards()),
            },
        }
    }

    fn count_energised(&self) -> usize {
        let energised_tiles: HashSet<(usize, usize)> =
            self.seen_beams.iter().map(|p| (p.row, p.column)).collect();

        energised_tiles.len()
    }

    pub fn calculate_energy_level(&mut self, position: Position) -> usize {
        self.seen_beams = HashSet::new();

        self.track_beam(Some(position));

        self.count_energised()
    }

    pub fn calculate_max_energy(&mut self) -> usize {
        let number_of_columns = self.schematic.first().unwrap().len();

        let mut starting_positions: Vec<Position> =
            Vec::with_capacity(number_of_columns * self.schematic.len());

        starting_positions.extend(self.schematic.first().unwrap().iter().enumerate().map(
            |(column, ..)| Position {
                row: 0,
                column,
                facing: Direction::South,
            },
        ));

        starting_positions.extend(self.schematic.last().unwrap().iter().enumerate().map(
            |(column, ..)| Position {
                row: self.schematic.len() - 1,
                column,
                facing: Direction::North,
            },
        ));

        starting_positions.extend(self.schematic.iter().enumerate().map(|(row, ..)| Position {
            row,
            column: 0,
            facing: Direction::East,
        }));

        starting_positions.extend(self.schematic.iter().enumerate().map(|(row, ..)| Position {
            row,
            column: number_of_columns - 1,
            facing: Direction::West,
        }));

        starting_positions
            .iter()
            .map(|&p| self.calculate_energy_level(p))
            .max()
            .unwrap()
    }
}

impl Display for Contraption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = self
            .schematic
            .iter()
            .map(|row| String::from_iter(row.iter().map(|tile| tile.to_char())))
            .collect();

        write!(f, "{}", lines.join("\n"))
    }
}
