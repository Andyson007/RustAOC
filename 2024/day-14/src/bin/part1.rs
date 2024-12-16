use std::{
    cmp::Ordering,
    convert::Infallible,
    ops::{Add, AddAssign, Mul, SubAssign},
    str::FromStr,
};

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input, (101, 103));
    println!("{ans}");
}

fn solve(raw: &str, size: (isize, isize)) -> usize {
    let mut robots = raw
        .lines()
        .map(Robot::from_str)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    for robot in &robots {
        println!("{robot:?}");
    }
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.tick(size);
        }
    }
    let mut counts = [0; 4];
    for i in 0..size.0 {
        for j in 0..size.1 {
            let count = robots.iter().filter(|x| x.pos == Pos::new(i, j)).count();
            if count == 0 {
                print!(".");
            } else {
                print!("{count}");
            }
            let mut idx = 0;
            match i.cmp(&((size.0 - 1) / 2)) {
                Ordering::Less => (),
                Ordering::Equal => continue,
                Ordering::Greater => idx += 1,
            }
            match j.cmp(&((size.1 - 1) / 2)) {
                Ordering::Less => (),
                Ordering::Equal => continue,
                Ordering::Greater => idx += 2,
            }
            counts[idx] += count;
        }
        println!();
    }
    counts.into_iter().reduce(std::ops::Mul::mul).unwrap()
}

#[derive(Debug)]
struct Robot {
    pos: Pos,
    velocitiy: Pos,
}

impl Robot {
    pub fn tick(&mut self, size: (isize, isize)) {
        self.pos += self.velocitiy;
        if self.pos.x < 0 {
            self.pos.x += size.0;
        } else if self.pos.x >= size.0 {
            self.pos.x -= size.0;
        }

        if self.pos.y < 0 {
            self.pos.y += size.1;
        } else if self.pos.y >= size.1 {
            self.pos.y -= size.1;
        }
    }
}

impl FromStr for Robot {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(' ').unwrap();
        let pos = Pos::from_str(&p[2..]).unwrap();
        let vel = Pos::from_str(&v[2..]).unwrap();
        Ok(Self {
            pos,
            velocitiy: vel,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
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

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        assert_eq!(solve(include_str!("../../example"), (11, 7)), 12);
    }
}
