use std::{cmp::Ordering, convert::Infallible, fmt::Display, str::FromStr};

use crate::{planet::Planet, vector::Vector};

#[derive(Debug, Clone, Eq)]
pub struct System<const N: usize> {
    pub planets: [Planet; N],
}

impl<const N: usize> PartialEq for System<N> {
    fn eq(&self, other: &Self) -> bool {
        self.planets == other.planets
    }
}

impl<const N: usize> Display for System<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for planet in &self.planets {
            writeln!(f, "{planet}")?;
        }
        Ok(())
    }
}

impl<const N: usize> FromStr for System<N> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            planets: s
                .lines()
                .map(Planet::from_str)
                .collect::<Result<Vec<_>, ()>>()
                .unwrap()
                .try_into()
                .unwrap(),
        })
    }
}

impl<const N: usize> System<N> {
    pub fn time_step(&mut self) {
        self.apply_gravity();
        self.apply_velocities();
    }
    fn apply_gravity(&mut self) {
        let mut to_apply = [Vector::new((0, 0, 0)); N];
        for (i, a) in self.planets.iter().enumerate() {
            for (j, b) in self.planets.iter().enumerate() {
                if i == j {
                    continue;
                }
                to_apply[i].x += to_val(a.pos.x, b.pos.x);
                to_apply[i].y += to_val(a.pos.y, b.pos.y);
                to_apply[i].z += to_val(a.pos.z, b.pos.z);
            }
        }

        for (planet, application) in self.planets.iter_mut().zip(to_apply) {
            planet.vel += application;
        }
    }

    fn apply_velocities(&mut self) {
        for planet in self.planets.iter_mut() {
            planet.apply_velocity();
        }
    }

    pub fn energy(&self) -> usize {
        self.planets
            .iter()
            .map(|x| x.pos.energy() * x.vel.energy())
            .sum()
    }
}

#[inline]
fn to_val(a: isize, b: isize) -> isize {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}
