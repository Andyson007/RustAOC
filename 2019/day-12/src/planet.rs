use std::{fmt::Display, str::FromStr};

use crate::vector::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Planet {
    pub pos: Vector,
    pub vel: Vector,
}

impl Display for Planet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos={}, vel={}", self.pos, self.vel)
    }
}

impl FromStr for Planet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(',')
            .map(|x| {
                x.chars()
                    .filter(|x| x.is_ascii_digit() || *x == '-')
                    .collect::<String>()
            })
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        Ok(Planet::new(Vector::new((values[0], values[1], values[2]))))
    }
}

impl Planet {
    pub fn new(pos: Vector) -> Self {
        Self {
            pos,
            vel: Vector::default(),
        }
    }

    pub fn apply_velocity(&mut self) {
        self.pos += self.vel;
    }
}
