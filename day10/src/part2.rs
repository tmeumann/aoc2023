use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq)]
enum RelativeDirection {
    Left,
    Right,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
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

    pub fn move_forwards(&self) -> Self {
        match self.facing {
            Direction::North => self.north(),
            Direction::South => self.south(),
            Direction::East => self.east(),
            Direction::West => self.west(),
        }
        .unwrap()
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

    fn left(&self) -> Option<Self> {
        match self.facing {
            Direction::North => self.west(),
            Direction::South => self.east(),
            Direction::East => self.north(),
            Direction::West => self.south(),
        }
    }

    fn right(&self) -> Option<Self> {
        match self.facing {
            Direction::North => self.east(),
            Direction::South => self.west(),
            Direction::East => self.south(),
            Direction::West => self.north(),
        }
    }

    fn is_same_tile(&self, other: &Option<Position>) -> bool {
        if let Some(other) = other {
            self.row == other.row && self.column == other.column
        } else {
            false
        }
    }
}

trait Map<T> {
    fn get_tile(&self, position: &Position) -> T;
}

impl<T> Map<T> for Vec<Vec<T>>
where
    T: Copy,
{
    fn get_tile(&self, position: &Position) -> T {
        self[position.row][position.column]
    }
}

trait ConnectedMap {
    fn get_connected_tiles(&self, position: &Position) -> (Option<Position>, Option<Position>);
    fn find_initial_connected_tiles(&self, starting_position: Position) -> (Position, Position);
}

impl ConnectedMap for Vec<Vec<char>> {
    fn get_connected_tiles(&self, position: &Position) -> (Option<Position>, Option<Position>) {
        let tile = self.get_tile(position);
        match tile {
            '|' => (position.north(), position.south()),
            '-' => (position.east(), position.west()),
            'L' => (position.north(), position.east()),
            'F' => (position.south(), position.east()),
            '7' => (position.south(), position.west()),
            'J' => (position.north(), position.west()),
            _ => (None, None),
        }
    }

    fn find_initial_connected_tiles(&self, starting_position: Position) -> (Position, Position) {
        let with_connections: Vec<Position> = [
            starting_position.north(),
            starting_position.south(),
            starting_position.east(),
            starting_position.west(),
        ]
        .into_iter()
        .filter(|position| {
            if let Some(position) = position {
                let (p1, p2) = self.get_connected_tiles(position);
                p1 == Some(starting_position) || p2 == Some(starting_position)
            } else {
                false
            }
        })
        .flatten()
        .collect();

        if with_connections.len() != 2 {
            panic!("Oops");
        }

        (with_connections[0], with_connections[1])
    }
}

#[derive(Debug)]
struct WorldMap {
    pipe_tiles: Vec<Vec<char>>,
    path_tiles: Vec<Vec<char>>,
    start: Position,
}

impl WorldMap {
    pub fn new(input: &str) -> Self {
        let mut pipe_tiles = input.lines().map(|line| line.chars().collect()).collect();

        let path_tiles = input
            .lines()
            .map(|line| vec![' '; line.chars().count()])
            .collect();

        let start = Self::find_starting_position(&mut pipe_tiles);

        Self {
            pipe_tiles,
            path_tiles,
            start,
        }
    }

    pub fn find_starting_position(pipe_tiles: &mut Vec<Vec<char>>) -> Position {
        for (row, row_tiles) in pipe_tiles.iter().enumerate() {
            for (column, c) in row_tiles.iter().enumerate() {
                if *c == 'S' {
                    let mut position = Position {
                        row,
                        column,
                        facing: Direction::North,
                    };

                    let (option1, option2) = pipe_tiles.find_initial_connected_tiles(position);

                    let mut turns_since_last_option = 10;

                    // this loop ensures the start pipe is either straight or a right-hand turn
                    loop {
                        let test_position = position.move_forwards();
                        if option1.is_same_tile(&Some(test_position))
                            || option2.is_same_tile(&Some(test_position))
                        {
                            if turns_since_last_option > 2 {
                                turns_since_last_option = 0;
                            } else {
                                break;
                            }
                        }
                        turns_since_last_option += 1;
                        position = position.turn_right();
                    }

                    return position;
                }
            }
        }
        panic!("Failed to find starting position");
    }

    fn turn_and_flood(
        &mut self,
        position: Position,
        left_turns: u16,
        right_turns: u16,
    ) -> RelativeDirection {
        let flood_direction = match self.pipe_tiles[position.row][position.column] {
            '|' => self.move_and_flood(position, left_turns, right_turns),
            '-' => self.move_and_flood(position, left_turns, right_turns),
            'L' => match position.facing {
                Direction::West => {
                    self.move_and_flood(position.turn_right(), left_turns, right_turns + 1)
                }
                Direction::South => {
                    self.move_and_flood(position.turn_left(), left_turns + 1, right_turns)
                }
                _ => panic!("Oops"),
            },
            'F' => match position.facing {
                Direction::West => {
                    self.move_and_flood(position.turn_left(), left_turns + 1, right_turns)
                }
                Direction::North => {
                    self.move_and_flood(position.turn_right(), left_turns, right_turns + 1)
                }
                _ => panic!("Oops"),
            },
            '7' => match position.facing {
                Direction::East => {
                    self.move_and_flood(position.turn_right(), left_turns, right_turns + 1)
                }
                Direction::North => {
                    self.move_and_flood(position.turn_left(), left_turns + 1, right_turns)
                }
                _ => panic!("Oops"),
            },
            'J' => match position.facing {
                Direction::East => {
                    self.move_and_flood(position.turn_left(), left_turns + 1, right_turns)
                }
                Direction::South => {
                    self.move_and_flood(position.turn_right(), left_turns, right_turns + 1)
                }
                _ => panic!("Oops"),
            },
            _ => panic!("Oops"),
        };
        // flood the place
        let position_to_flood = match flood_direction {
            RelativeDirection::Left => position.left(),
            RelativeDirection::Right => position.right(),
        };
        if self.path_tiles[position_to_flood.unwrap().row][position_to_flood.unwrap().column] == ' '
        {
            self.flood(position_to_flood.unwrap());
        };
        flood_direction
    }

    fn move_and_flood(
        &mut self,
        position: Position,
        left_turns: u16,
        right_turns: u16,
    ) -> RelativeDirection {
        let new_position = position.move_forwards();
        self.path_tiles[new_position.row][new_position.column] = '+';

        let flood_direction = if new_position.is_same_tile(&Some(self.start)) {
            if left_turns > right_turns {
                RelativeDirection::Left
            } else {
                RelativeDirection::Right
            }
        } else {
            self.turn_and_flood(new_position, left_turns, right_turns)
        };

        // flood the place
        let new_position_to_flood = match flood_direction {
            RelativeDirection::Left => new_position.left(),
            RelativeDirection::Right => new_position.right(),
        };
        if self.path_tiles[new_position_to_flood.unwrap().row]
            [new_position_to_flood.unwrap().column]
            == ' '
        {
            self.flood(new_position_to_flood.unwrap());
        };
        let old_position_to_flood = match flood_direction {
            RelativeDirection::Left => position.left(),
            RelativeDirection::Right => position.right(),
        };
        if self.path_tiles[old_position_to_flood.unwrap().row]
            [old_position_to_flood.unwrap().column]
            == ' '
        {
            self.flood(old_position_to_flood.unwrap());
        };

        flood_direction
    }

    fn flood(&mut self, position: Position) {
        if self.path_tiles[position.row][position.column] == ' ' {
            self.path_tiles[position.row][position.column] = '#';
            self.flood(position.north().unwrap());
            self.flood(position.south().unwrap());
            self.flood(position.east().unwrap());
            self.flood(position.west().unwrap());
        }
    }
}

impl Display for WorldMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.path_tiles {
            writeln!(f, "{}", String::from_iter(row))?;
        }
        Ok(())
    }
}

pub fn solve_part_2(input: String) -> usize {
    // build a map with JUST the path, tracking turns
    // if more total right turns than left, colour left, otherwise right

    let mut world_map = WorldMap::new(&input);

    world_map.move_and_flood(world_map.start, 0, 0);

    println!("{world_map}");

    world_map
        .path_tiles
        .iter()
        .flatten()
        .filter(|&&c| c == '#')
        .count()
}

#[cfg(test)]
mod test {
    use crate::part2::solve_part_2;
    use std::fs::read_to_string;

    #[test]
    fn test_1() {
        let input = read_to_string("test1.txt").expect("Failed to read test file");

        let solution = solve_part_2(input);

        assert_eq!(solution, 4);
    }

    #[test]
    fn test_2() {
        let input = read_to_string("test2.txt").expect("Failed to read test file");

        let solution = solve_part_2(input);

        assert_eq!(solution, 8);
    }
}
