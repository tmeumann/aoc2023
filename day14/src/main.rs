use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::fs::read_to_string;

#[derive(Eq, PartialEq, Hash)]
enum Tile {
    SquareRock,
    RoundRock,
    Empty,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, ()> {
        match value {
            '#' => Ok(Self::SquareRock),
            'O' => Ok(Self::RoundRock),
            '.' => Ok(Self::Empty),
            _ => Err(()),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SquareRock => f.write_char('#'),
            Self::RoundRock => f.write_char('O'),
            Self::Empty => f.write_char('.'),
        }
    }
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Self::SquareRock => '#',
            Self::RoundRock => 'O',
            Self::Empty => '.',
        }
    }
}

struct Dish {
    map: Vec<Vec<Tile>>,
}

impl Display for Dish {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = self
            .map
            .iter()
            .map(|row| String::from_iter(row.iter().map(|tile| tile.to_char())))
            .collect();

        write!(f, "{}", lines.join("\n"))
    }
}

impl Dish {
    fn new(input: &str) -> Self {
        let map: Vec<Vec<Tile>> = input
            .lines()
            .map(|line| line.chars().flat_map(Tile::try_from).collect())
            .collect();

        Self { map }
    }

    fn tilt_north(&mut self) {
        loop {
            let mut something_moved = false;

            for row_index in 1..self.map.len() {
                for column_index in 0..self.map[row_index].len() {
                    if Tile::RoundRock == self.map[row_index][column_index]
                        && Tile::Empty == self.map[row_index - 1][column_index]
                    {
                        self.map[row_index - 1][column_index] = Tile::RoundRock;
                        self.map[row_index][column_index] = Tile::Empty;
                        something_moved = true;
                    }
                }
            }

            if !something_moved {
                break;
            }
        }
    }

    fn tilt_west(&mut self) {
        loop {
            let mut something_moved = false;

            for row_index in 0..self.map.len() {
                for column_index in 1..self.map[row_index].len() {
                    if Tile::RoundRock == self.map[row_index][column_index]
                        && Tile::Empty == self.map[row_index][column_index - 1]
                    {
                        self.map[row_index][column_index - 1] = Tile::RoundRock;
                        self.map[row_index][column_index] = Tile::Empty;
                        something_moved = true;
                    }
                }
            }

            if !something_moved {
                break;
            }
        }
    }

    fn tilt_south(&mut self) {
        loop {
            let mut something_moved = false;

            for row_index in (0..self.map.len() - 1).rev() {
                for column_index in 0..self.map[row_index].len() {
                    if Tile::RoundRock == self.map[row_index][column_index]
                        && Tile::Empty == self.map[row_index + 1][column_index]
                    {
                        self.map[row_index + 1][column_index] = Tile::RoundRock;
                        self.map[row_index][column_index] = Tile::Empty;
                        something_moved = true;
                    }
                }
            }

            if !something_moved {
                break;
            }
        }
    }

    fn tilt_east(&mut self) {
        loop {
            let mut something_moved = false;

            for row_index in 0..self.map.len() {
                for column_index in (0..self.map[row_index].len() - 1).rev() {
                    if Tile::RoundRock == self.map[row_index][column_index]
                        && Tile::Empty == self.map[row_index][column_index + 1]
                    {
                        self.map[row_index][column_index + 1] = Tile::RoundRock;
                        self.map[row_index][column_index] = Tile::Empty;
                        something_moved = true;
                    }
                }
            }

            if !something_moved {
                break;
            }
        }
    }

    fn spin(&mut self, times: usize) {
        let mut history: HashMap<String, usize> = HashMap::new();

        for i in 0..times {
            let current_hash_key = format!("{}", self);

            if let Some(j) = history.get(&current_hash_key) {
                let loop_length = i - j;
                let remaining_iterations = times - i;

                if remaining_iterations % loop_length == 0 {
                    return;
                }
            }

            history.insert(current_hash_key, i);

            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();
        }
    }

    fn total_load(&self) -> usize {
        self.map
            .iter()
            .rev()
            .enumerate()
            .map(|(i, row)| (i + 1, row))
            .map(|(i, row)| row.iter().filter(|&t| t == &Tile::RoundRock).count() * i)
            .sum()
    }
}

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut dish = Dish::new(&input);

    dish.spin(1_000_000_000);

    let total_load = dish.total_load();

    println!("{total_load}")
}
