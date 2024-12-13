use std::ops::{Add, AddAssign, Mul, SubAssign};

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}")
}

fn solve(raw: &str) -> usize {
    raw.split("\n\n")
        .map(Game::new)
        .map(|mut x| {
            x.prize += Pos {
                x: 10000000000000,
                y: 10000000000000,
            };
            x
        })
        .map(|x| x.solve())
        .inspect(|x| println!("{x:?}"))
        .map(|x| x.unwrap_or(0))
        .sum()
}

struct Game {
    a: Pos,
    b: Pos,
    prize: Pos,
}

impl Game {
    pub fn new(raw: &str) -> Self {
        let mut iter = raw.lines();
        let a = Pos::from_str(iter.next().unwrap().split_once(": ").unwrap().1, '+');
        let b = Pos::from_str(iter.next().unwrap().split_once(": ").unwrap().1, '+');
        let prize = Pos::from_str(iter.next().unwrap().split_once(": ").unwrap().1, '=');
        Self { a, b, prize }
    }

    pub fn solve(&self) -> Option<usize> {
        #[allow(non_snake_case)]
        let A = self.a;
        #[allow(non_snake_case)]
        let B = self.b;
        #[allow(non_snake_case)]
        let P = self.prize;
        let numerator = P.y * A.x - A.y * P.x;
        let denomenator = B.y * A.x - B.x * A.y;
        if numerator % denomenator != 0 {
            return None;
        }
        let b = numerator / denomenator;
        if b < 0 {
            return None;
        }
        let numerator = P.x - b * B.x;
        let denomenator = self.a.x;
        if numerator % denomenator != 0 {
            return None;
        }
        let a = numerator / denomenator;
        if a < 0 {
            return None;
        }
        Some(3 * (a as usize) + (b as usize))
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
    pub fn from_str(raw: &str, delim: char) -> Self {
        let (l, r) = raw.split_once(", ").unwrap();
        Self {
            x: l.split_once(delim).unwrap().1.parse().unwrap(),
            y: r.split_once(delim).unwrap().1.parse().unwrap(),
        }
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
        assert_eq!(solve(include_str!("../../example")), 480)
    }
}
