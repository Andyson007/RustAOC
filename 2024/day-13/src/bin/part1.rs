use std::ops::{Add, AddAssign, Mul, SubAssign};

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}")
}

fn solve(raw: &str) -> usize {
    raw.split("\n\n")
        .map(Game::new)
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
        let start_amount = self.prize.x / self.b.x;
        let mut curr = self.b * start_amount;
        if curr == self.prize {
            return Some(start_amount as usize);
        }
        curr += self.b;
        let mut cost = start_amount + 1;
        let mut a_count = 0;
        let mut b_count = cost;
        loop {
            if curr == self.prize && a_count < 100 && b_count < 100 {
                return Some(cost as usize);
            }
            if curr.x < self.prize.x {
                if a_count >= 100 {
                    return None;
                }
                curr += self.a;
                cost += 3;
                a_count += 1;
            } else {
                curr -= self.b;
                cost -= 1;
                b_count -= 1;
                if b_count < 0 {
                    return None;
                }
            }
        }
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
