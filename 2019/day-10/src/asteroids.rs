use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    str::FromStr,
};

use crate::direction::Direction;

pub struct AsteroidField {
    asteroids: Vec<(isize, isize)>,
}

impl FromStr for AsteroidField {
    type Err = Infallible;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            asteroids: input
                .lines()
                .enumerate()
                .flat_map(|(i, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, x)| *x != '.')
                        .map(move |(j, _)| (j as isize, i as isize))
                })
                .collect(),
        })
    }
}

impl AsteroidField {
    pub fn evaluate(&self, x: isize, y: isize) -> usize {
        debug_assert!(self.asteroids.contains(&(x, y)));
        let mut directions = HashSet::new();
        for (ax, ay) in &self.asteroids {
            directions.insert(Direction::new(ax - x, ay - y));
        }
        // -1 because self is also included
        directions.len() - 1
    }

    pub fn evaluate_all(&self) -> Option<((isize, isize), usize)> {
        self.asteroids
            .iter()
            .map(|(ax, ay)| ((*ax, *ay), self.evaluate(*ax, *ay)))
            .max_by_key(|x| x.1)
    }

    pub fn get_all_asteroids(&self, x: isize, y: isize) -> Vec<(Direction, usize)> {
        let mut all = self
            .asteroids
            .iter()
            .map(|(ax, ay)| Direction::new(ax - x, ay - y))
            .filter(|x| !x.is_zero())
            .fold(HashMap::<_, usize>::new(), |mut sum, curr| {
                *sum.entry(curr).or_default() += 1;
                sum
            })
            .into_iter()
            .collect::<Vec<_>>();
        all.sort_by_key(|x| x.0);
        all
    }

    pub fn get_nth_with_dir(
        &self,
        from: (isize, isize),
        dir: Direction,
        mut nth: usize,
    ) -> (isize, isize) {
        let mut curr = from;
        while nth != 0 {
            curr.0 += dir.x;
            curr.1 += dir.y;
            if self.asteroids.contains(&curr) {
                nth -= 1;
            }
        }
        curr
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::AsteroidField;

    #[test]
    fn asteroid_empty() {
        let asteroid = AsteroidField::from_str(
            "............
............",
        )
        .unwrap();
        assert!(asteroid.asteroids.is_empty(), "{:?}", asteroid.asteroids)
    }

    #[test]
    fn asteroid() {
        let asteroids = AsteroidField::from_str(
            "............
.........#..",
        )
        .unwrap();
        assert_eq!(asteroids.asteroids.len(), 1, "{:?}", asteroids.asteroids);
        assert!(
            asteroids.asteroids.contains(&(9, 1)),
            "{:?}",
            asteroids.asteroids
        );
    }

    #[test]
    fn single_eval() {
        let asteroid_field = AsteroidField::from_str(
            "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####",
        )
        .unwrap();
        let amount = asteroid_field.evaluate(5, 8);
        assert_eq!(amount, 33);
    }

    #[test]
    fn eval_all() {
        let asteroid_field = AsteroidField::from_str(
            "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####",
        )
        .unwrap();
        let amount = asteroid_field.evaluate_all().unwrap();
        assert_eq!(amount, ((5, 8), 33));
    }
    #[test]
    fn big_all() {
        let asteroid_field = AsteroidField::from_str(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
        )
        .unwrap();
        let amount = asteroid_field.evaluate_all().unwrap();
        assert_eq!(amount, ((11, 13), 210));
    }
}
