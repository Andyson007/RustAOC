use std::{
    convert::Infallible,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl Pos {
    pub const ZERO: Pos = Pos::new(0, 0);
    pub const NORTH: Pos = Pos::new(0, 1);
    pub const EAST: Pos = Pos::new(1, 0);
    pub const SOUTH: Pos = Pos::new(0, -1);
    pub const WEST: Pos = Pos::new(-1, 0);
    
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn dist(&self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Neg for Pos {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl FromStr for Pos {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

impl Mul<isize> for Pos {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add<Pos> for Pos {
    type Output = Self;

    fn add(self, rhs: Pos) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Pos> for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub<Pos> for Pos {
    type Output = Self;

    fn sub(self, rhs: Pos) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
