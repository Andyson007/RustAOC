use std::{
    fmt::Display,
    ops::{Add, AddAssign, Mul},
};

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<x={:3}, y={:3}, z={:3}>", self.x, self.y, self.z)
    }
}

impl Mul for Vector {
    type Output = isize;
    fn mul(self, other: Self) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Vector {
    pub fn new(vec: (isize, isize, isize)) -> Self {
        Self {
            x: vec.0,
            y: vec.1,
            z: vec.2,
        }
    }

    pub fn energy(&self) -> usize {
        self.x.unsigned_abs() + self.y.unsigned_abs() + self.z.unsigned_abs()
    }
}
