use ascii::{AsciiChar, AsciiString};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

struct Instruction {
    direction: Direction,
    distance: u8,
}

impl TryFrom<String> for Instruction {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let sections: Vec<&str> = value.split_whitespace().collect();

        let direction = (*sections.first().ok_or(())?).try_into()?;

        let distance = sections.get(1).ok_or(())?.parse::<u8>().map_err(|_| ())?;

        Ok(Self {
            direction,
            distance,
        })
    }
}

pub struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn up(&self) -> Option<Self> {
        if self.row > 0 {
            Some(Position {
                row: self.row - 1,
                column: self.column,
            })
        } else {
            None
        }
    }

    fn down(&self) -> Option<Self> {
        Some(Position {
            row: self.row + 1,
            column: self.column,
        })
    }

    fn right(&self) -> Option<Self> {
        Some(Position {
            row: self.row,
            column: self.column + 1,
        })
    }

    fn left(&self) -> Option<Self> {
        if self.column > 0 {
            Some(Position {
                row: self.row,
                column: self.column - 1,
            })
        } else {
            None
        }
    }
}

struct MapBuilder {
    map: VecDeque<VecDeque<AsciiChar>>,
    position: Position,
}

impl Display for MapBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            writeln!(f, "{}", AsciiString::from_iter(row))?;
        }
        Ok(())
    }
}

// use vecdeques
// dig up => add a # to the vd above, or create vd if not exists & then add
// dig down => add a # to the vd above, or create vs if not exists & then add
// dig right => set right to #, or append # and append empty to all other arrays
// dig left => set left to #, or prepend # and prepend empty to all other arrays
impl MapBuilder {
    fn new() -> Self {
        let map = VecDeque::from([VecDeque::from([AsciiChar::Hash])]);
        let position = Position { row: 0, column: 0 };

        Self { map, position }
    }

    fn follow_instruction(&mut self, instruction: &Instruction) {
        match instruction.direction {
            Direction::Up => self.dig_up(instruction.distance),
            Direction::Down => self.dig_down(instruction.distance),
            Direction::Left => self.dig_left(instruction.distance),
            Direction::Right => self.dig_right(instruction.distance),
        }
    }

    fn dig_up(&mut self, distance: u8) {
        for _ in 0..distance {
            if self.position.row > 0 {
                self.position.row -= 1;
            } else {
                self.map.push_front(self.build_row());
            }
            self.dig();
        }
    }

    fn dig_down(&mut self, distance: u8) {
        for _ in 0..distance {
            if self.position.row >= self.map.len() - 1 {
                self.map.push_back(self.build_row());
            }
            self.position.row += 1;
            self.dig();
        }
    }

    fn dig_left(&mut self, distance: u8) {
        for _ in 0..distance {
            if self.position.column > 0 {
                self.position.column -= 1;
            } else {
                for row in &mut self.map {
                    row.push_front(AsciiChar::Space)
                }
            }
            self.dig();
        }
    }

    fn dig_right(&mut self, distance: u8) {
        for _ in 0..distance {
            if self.position.column >= self.map[0].len() - 1 {
                for row in &mut self.map {
                    row.push_back(AsciiChar::Space)
                }
            }
            self.position.column += 1;
            self.dig();
        }
    }

    fn dig(&mut self) {
        self.map[self.position.row][self.position.column] = AsciiChar::Hash;
    }

    fn build_row(&self) -> VecDeque<AsciiChar> {
        let row_length = self.map[0].len();

        let mut row = VecDeque::with_capacity(row_length);

        for _ in 0..row_length {
            row.push_back(AsciiChar::Space);
        }
        row
    }

    fn mark_ground(&mut self) {
        for column in 0..self.map[0].len() {
            self.flood_from(Position { row: 0, column }, AsciiChar::Dot);
            self.flood_from(
                Position {
                    row: self.map.len() - 1,
                    column,
                },
                AsciiChar::Dot,
            );
        }
        for row in 0..self.map.len() {
            self.flood_from(Position { row, column: 0 }, AsciiChar::Dot);
            self.flood_from(
                Position {
                    row,
                    column: self.map[row].len() - 1,
                },
                AsciiChar::Dot,
            );
        }
    }

    fn excavate(&mut self) {
        for row in self.map.iter_mut() {
            for tile in row.iter_mut() {
                if *tile == AsciiChar::Space {
                    *tile = AsciiChar::Hash;
                }
            }
        }
    }

    fn flood_from(&mut self, position: Position, character: AsciiChar) {
        if let Some(tile) = self
            .map
            .get_mut(position.row)
            .and_then(|r| r.get_mut(position.column))
        {
            if *tile == AsciiChar::Space {
                *tile = character;
                [
                    position.up(),
                    position.down(),
                    position.left(),
                    position.right(),
                ]
                .into_iter()
                .flatten()
                .for_each(|p| self.flood_from(p, character));
            }
        }
    }

    fn volume(&self) -> usize {
        self.map
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|&&character| character == AsciiChar::Hash)
                    .count()
            })
            .sum()
    }
}

fn main() {
    let mut map_builder = MapBuilder::new();

    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map_while(|l| l.ok())
        .flat_map(Instruction::try_from)
        .for_each(|i| map_builder.follow_instruction(&i));

    map_builder.mark_ground();
    map_builder.excavate();

    let lagoon_volume = map_builder.volume();

    println!("{map_builder}");
    println!("{lagoon_volume}")
}
