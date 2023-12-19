use std::fmt::{Display, Formatter, Write};

pub enum Tilt {
    Right,
    Left,
}

pub enum Orientation {
    Vertical,
    Horizontal,
}

pub enum Tile {
    Empty,
    Mirror(Tilt),
    Splitter(Orientation),
}

impl Tile {
    pub fn new(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '\\' => Self::Mirror(Tilt::Left),
            '/' => Self::Mirror(Tilt::Right),
            '|' => Self::Splitter(Orientation::Vertical),
            '-' => Self::Splitter(Orientation::Horizontal),
            _ => panic!("Unexpected input"),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Mirror(Tilt::Left) => '\\',
            Self::Mirror(Tilt::Right) => '/',
            Self::Splitter(Orientation::Vertical) => '|',
            Self::Splitter(Orientation::Horizontal) => '-',
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_char())
    }
}
