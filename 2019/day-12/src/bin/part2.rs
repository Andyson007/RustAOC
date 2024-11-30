use day_12::lcm;
use std::{cmp::Ordering, fmt::Display, str::FromStr};

use day_12::system::System as CrateSystem;
fn main() {
    let system = CrateSystem::<4>::from_str(include_str!("../../input")).unwrap();
    let systems = [
        System {
            planets: system.planets.map(|x| (x.pos.x, x.vel.x)),
        },
        System {
            planets: system.planets.map(|x| (x.pos.y, x.vel.y)),
        },
        System {
            planets: system.planets.map(|x| (x.pos.z, x.vel.z)),
        },
    ];
    let a = solve_dimension(systems[0].clone());
    let b = solve_dimension(systems[1].clone());
    let c = solve_dimension(systems[2].clone());
    println!("{a} {b} {c}");
    let ans = lcm(lcm(a, b), c);
    println!("{ans}");
}

fn solve_dimension<const N: usize>(system: System<N>) -> usize {
    let mut mutating = system.clone();
    for i in 1.. {
        mutating.time_step();
        if mutating == system {
            return i;
        }
    }
    panic!()
}

#[derive(Debug, Clone, Eq)]
pub struct System<const N: usize> {
    pub planets: [(isize, isize); N],
}

impl<const N: usize> PartialEq for System<N> {
    fn eq(&self, other: &Self) -> bool {
        self.planets == other.planets
    }
}

impl<const N: usize> Display for System<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for planet in &self.planets {
            write!(f, "pos={}, vel={}", planet.0, planet.1)?;
        }
        Ok(())
    }
}

impl<const N: usize> System<N> {
    pub fn time_step(&mut self) {
        self.apply_gravity();
        self.apply_velocities();
    }
    fn apply_gravity(&mut self) {
        let mut to_apply = [0; N];
        for (i, a) in self.planets.iter().enumerate() {
            for b in self.planets.iter() {
                to_apply[i] += match a.0.cmp(&b.0) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };
            }
        }

        for (planet, application) in self.planets.iter_mut().zip(to_apply) {
            planet.1 += application;
        }
    }

    fn apply_velocities(&mut self) {
        for planet in self.planets.iter_mut() {
            planet.0 += planet.1
        }
    }

    pub fn energy(&self) -> usize {
        self.planets
            .iter()
            .map(|(pos, vel)| pos.unsigned_abs() * vel.unsigned_abs())
            .sum()
    }
}

fn calc_energy<const N: usize, const M: usize>(systems: [System<N>; M]) -> usize {
    let mut ret = 0;
    for planet in 0..N {
        let mut pot = 0;
        let mut kin = 0;
        for system in &systems {
            pot += system.planets[planet].0.unsigned_abs();
            kin += system.planets[planet].1.unsigned_abs();
        }
        ret += pot * kin;
    }
    ret
}

#[cfg(test)]
mod test {
    use crate::{calc_energy, solve_dimension};
    use std::str::FromStr;

    use day_12::{lcm, system::System as CrateSystem};

    use crate::System;

    #[test]
    fn part1_example1() {
        let system = CrateSystem::<4>::from_str(
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
        )
        .unwrap();
        let mut systems = [
            System {
                planets: system.planets.map(|x| (x.pos.x, x.vel.x)),
            },
            System {
                planets: system.planets.map(|x| (x.pos.y, x.vel.y)),
            },
            System {
                planets: system.planets.map(|x| (x.pos.z, x.vel.z)),
            },
        ];
        println!("{systems:?}");
        for _ in 0..10 {
            for i in 0..4 {
                println!(
                    "pos=<x={:3}, y={:3}, z={:3}> vel=<x={:3}, y={:3}, z={:3}>",
                    systems[0].planets[i].0,
                    systems[1].planets[i].0,
                    systems[2].planets[i].0,
                    systems[0].planets[i].1,
                    systems[1].planets[i].1,
                    systems[2].planets[i].1,
                );
            }
            systems[0].time_step();
            systems[1].time_step();
            systems[2].time_step();
            println!()
        }

        let energy = calc_energy(systems);
        assert_eq!(energy, 179);
    }

    #[test]
    fn part1() {
        let system = CrateSystem::<4>::from_str(include_str!("../../input")).unwrap();
        let mut systems = [
            System {
                planets: system.planets.map(|x| (x.pos.x, x.vel.x)),
            },
            System {
                planets: system.planets.map(|x| (x.pos.y, x.vel.y)),
            },
            System {
                planets: system.planets.map(|x| (x.pos.z, x.vel.z)),
            },
        ];
        for _ in 0..1000 {
            systems[0].time_step();
            systems[1].time_step();
            systems[2].time_step();
        }

        assert_eq!(calc_energy(systems), 12070);
    }

    #[test]
    fn part2_example2() {
        let system = CrateSystem::<4>::from_str(
            "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>",
        )
        .unwrap();
        let systems = [
            System {
                planets: system.planets.map(|x| (x.pos.x, x.vel.x)),
            },
            System {
                planets: system.planets.map(|x| (x.pos.y, x.vel.y)),
            },
            System {
                planets: system.planets.map(|x| (x.pos.z, x.vel.z)),
            },
        ];
        let a = solve_dimension(systems[0].clone());
        let b = solve_dimension(systems[1].clone());
        let c = solve_dimension(systems[2].clone());
        let ans = lcm(lcm(a, b), c);
        assert_eq!(ans, 4686774924);
    }
}
