pub mod intcode;
#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
}

impl From<isize> for Tile {
    fn from(value: isize) -> Self {
        match value {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::HorizontalPaddle,
            4 => Self::Ball,
            _ => panic!(),
        }
    }
}
