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
struct Position {
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

    pub fn calculate_energy_level(&mut self) -> usize {
        let beam = Position {
            row: 0,
            column: 0,
            facing: Direction::East,
        };

        self.track_beam(Some(beam));

        self.count_energised()
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
