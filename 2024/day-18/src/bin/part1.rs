use std::{
    collections::{BinaryHeap, HashSet},
    convert::Infallible,
    ops::{Add, AddAssign, Mul, Neg, SubAssign},
    str::FromStr,
};

fn main() {
    let input = include_str!("../../input");
    let ans = solve::<71, 1024>(input);
    println!("{ans}")
}

fn solve<const N: usize, const AMOUNT: usize>(raw: &str) -> usize {
    let mut grid = [[true; N]; N];
    for (x, y) in raw.lines().map(|line| {
        let split = line.split_once(',').unwrap();
        (
            split.0.parse::<usize>().unwrap(),
            split.1.parse::<usize>().unwrap(),
        )
    })
    .take(AMOUNT)
    {
        grid[y][x] = false;
    }

    let mut poses = BinaryHeap::from([Node {
        pos: Pos::new(0, 0),
        dist: 0,
        dest: Pos::new(N as isize - 1, N as isize - 1),
    }]);

    let mut visited = HashSet::new();
    while let Some(next) = poses.pop() {
        if next.pos == next.dest {
            return next.dist;
        }
        if !visited.insert(next.pos) {
            continue;
        };
        println!("{next:?}");
        for dir in [
            Pos::new(1, 0),
            Pos::new(-1, 0),
            Pos::new(0, 1),
            Pos::new(0, -1),
        ] {
            let new_pos = next.pos + dir;
            if new_pos.x < 0 || new_pos.y < 0 || new_pos.x == N as isize || new_pos.y == N as isize
            {
                continue;
            }
            if grid[new_pos.y as usize][new_pos.x as usize] {
                poses.push(Node {
                    pos: new_pos,
                    dist: next.dist + 1,
                    dest: next.dest,
                });
            }
        }
    }

    for (i, x) in grid.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            print!(
                "{}",
                if visited.contains(&Pos::new(j as isize, i as isize)) {
                    '#'
                } else {
                    ' '
                }
            );
        }
        println!();
    }
    unreachable!()
}
#[derive(Debug, PartialEq, Eq)]
struct Node {
    pos: Pos,
    dist: usize,
    dest: Pos,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.min_dist().cmp(&(self.min_dist()))
    }
}

impl Node {
    pub fn min_dist(&self) -> usize {
        self.dist + self.dest.x.abs_diff(self.pos.x) + self.dest.y.abs_diff(self.pos.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
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

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        assert_eq!(solve::<7, 12>(include_str!("../../example")), 22)
    }
}
