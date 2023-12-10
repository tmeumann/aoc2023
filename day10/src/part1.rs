type Position = (usize, usize); // row_num, column_num

trait Navigable
where
    Self: Sized,
{
    fn north(&self) -> Option<Self>;
    fn south(&self) -> Option<Self>;
    fn east(&self) -> Option<Self>;
    fn west(&self) -> Option<Self>;
}

impl Navigable for Position {
    fn north(&self) -> Option<Self> {
        if self.0 > 0 {
            Some((self.0 - 1, self.1))
        } else {
            None
        }
    }

    fn south(&self) -> Option<Self> {
        Some((self.0 + 1, self.1))
    }

    fn east(&self) -> Option<Self> {
        Some((self.0, self.1 + 1))
    }

    fn west(&self) -> Option<Self> {
        if self.1 > 0 {
            Some((self.0, self.1 - 1))
        } else {
            None
        }
    }
}

type Breadcrumbs = Vec<Vec<bool>>;

type TileMap = Vec<Vec<char>>;

trait Map<T> {
    fn get_tile(&self, position: Position) -> T;
}

impl<T> Map<T> for Vec<Vec<T>>
where
    T: Copy,
{
    fn get_tile(&self, position: Position) -> T {
        self[position.0][position.1]
    }
}

trait ConnectedMap {
    fn get_connected_tiles(&self, position: Position) -> (Option<Position>, Option<Position>);
    fn find_starting_position(&self) -> Position;
    fn find_initial_connected_tiles(&self, starting_position: Position) -> (Position, Position);
    fn follow_path(
        &self,
        starting_position: Position,
        previous_position: Position,
        current_position: Position,
    ) -> usize;
}

impl ConnectedMap for Vec<Vec<char>> {
    fn get_connected_tiles(&self, position: Position) -> (Option<Position>, Option<Position>) {
        let (row_num, column_num) = position;
        let tile = self.get_tile(position);
        match tile {
            '|' => (
                if row_num > 0 {
                    Some((row_num - 1, column_num))
                } else {
                    None
                },
                Some((row_num + 1, column_num)),
            ),
            '-' => (
                if column_num > 0 {
                    Some((row_num, column_num - 1))
                } else {
                    None
                },
                Some((row_num, column_num + 1)),
            ),
            'L' => (
                if row_num > 0 {
                    Some((row_num - 1, column_num))
                } else {
                    None
                },
                Some((row_num, column_num + 1)),
            ),
            'F' => (
                Some((row_num, column_num + 1)),
                Some((row_num + 1, column_num)),
            ),
            '7' => (
                if column_num > 0 {
                    Some((row_num, column_num - 1))
                } else {
                    None
                },
                Some((row_num + 1, column_num)),
            ),
            'J' => (
                if column_num > 0 {
                    Some((row_num, column_num - 1))
                } else {
                    None
                },
                if row_num > 0 {
                    Some((row_num - 1, column_num))
                } else {
                    None
                },
            ),
            _ => (None, None),
        }
    }

    fn find_starting_position(&self) -> Position {
        for (row_num, row) in self.iter().enumerate() {
            for (column_num, character) in row.iter().enumerate() {
                if *character == 'S' {
                    return (row_num, column_num);
                }
            }
        }
        panic!("Failed to find starting position");
    }

    fn find_initial_connected_tiles(&self, starting_position: Position) -> (Position, Position) {
        let with_connections: Vec<Position> = [
            starting_position.north(),
            starting_position.south(),
            starting_position.east(),
            starting_position.west(),
        ]
        .iter()
        .filter(|&&position| {
            if let Some(position) = position {
                let (p1, p2) = self.get_connected_tiles(position);
                p1 == Some(starting_position) || p2 == Some(starting_position)
            } else {
                false
            }
        })
        .flatten()
        .copied()
        .collect();

        if with_connections.len() != 2 {
            panic!("Oops");
        }

        (with_connections[0], with_connections[1])
    }

    fn follow_path(
        &self,
        starting_position: Position,
        previous_position: Position,
        current_position: Position,
    ) -> usize {
        let (t1, t2) = self.get_connected_tiles(current_position);

        let tiles: Vec<Position> = [t1, t2]
            .iter()
            .filter_map(|&t| {
                if t == Some(starting_position) || t == Some(previous_position) {
                    None
                } else {
                    t
                }
            })
            .collect();

        if let Some(next_tile) = tiles.first() {
            1 + self.follow_path(starting_position, current_position, *next_tile)
        } else {
            1
        }
    }
}

pub fn solve_part_1(input: String) -> usize {
    let map: TileMap = input.lines().map(|line| line.chars().collect()).collect();

    let _breadcrumbs: Breadcrumbs = input
        .lines()
        .map(|line| vec![false; line.chars().count()])
        .collect();

    let starting_position = map.find_starting_position();

    let initial_connected_tiles = map.find_initial_connected_tiles(starting_position);

    let total = 1 + map.follow_path(
        starting_position,
        starting_position,
        initial_connected_tiles.0,
    );

    total / 2
}

#[cfg(test)]
mod test {
    use crate::part1::solve_part_1;
    use std::fs::read_to_string;

    #[test]
    fn test_1() {
        let input = read_to_string("test1.txt").expect("Failed to read test file");

        let solution = solve_part_1(input);

        assert_eq!(solution, 4);
    }

    #[test]
    fn test_2() {
        let input = read_to_string("test2.txt").expect("Failed to read test file");

        let solution = solve_part_1(input);

        assert_eq!(solution, 8);
    }
}
