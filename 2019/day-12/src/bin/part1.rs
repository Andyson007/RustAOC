use std::str::FromStr;

use day_12::system::System;

fn main() {
    let mut system = System::<4>::from_str(include_str!("../../input")).unwrap();
    for _ in 0..1000 {
        system.time_step();
    }
    println!("{}", system.energy());
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use day_12::{planet::Planet, system::System, vector::Vector};

    #[test]
    fn example1() {
        let mut system = System {
            planets: [
                Planet::new(Vector::new((-1, 0, 2))),
                Planet::new(Vector::new((2, -10, -7))),
                Planet::new(Vector::new((4, -8, 8))),
                Planet::new(Vector::new((3, 5, -1))),
            ],
        };
        for _ in 0..10 {
            system.time_step();
        }
        assert_eq!(system.energy(), 179);
    }

    #[test]
    fn example2() {
        let mut system = System::<4>::from_str(
            "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>",
        )
        .unwrap();
        for _ in 0..100 {
            system.time_step();
        }
        assert_eq!(system.energy(), 1940);
    }
}
