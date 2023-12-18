use std::fmt::{Display, Formatter, Write};
use std::fs::read_to_string;

#[derive(Eq, PartialEq)]
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

    fn shake_north(&mut self) {
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
    dish.shake_north();
    let total_load = dish.total_load();

    println!("{total_load}")
}
