use std::collections::HashMap;
use std::convert::Infallible;
use std::ops::Neg;
use std::{
    ops::{Add, AddAssign, Mul, SubAssign},
    str::FromStr,
};

fn main() {
    let input = include_str!("../../input");
    let ans = solve::<100>(input);
    println!("{ans}");
}

fn solve<const M: usize>(raw: &str) -> usize {
    let mut start = None;
    let grid = raw
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, x)| {
                    if x == 'S' {
                        start = Some(Pos::new(j as isize, i as isize));
                        '.'
                    } else {
                        x
                    }
                })
                .map(GridPos::from_char)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut dists = HashMap::new();
    let mut curr = start.unwrap();
    let mut curr_score = 0;
    loop {
        dists.insert(curr, curr_score);
        if grid[curr.y as usize][curr.x as usize] == GridPos::End {
            break;
        }

        let mut new_curr = None;
        for dir in [
            Pos::new(1, 0),
            Pos::new(-1, 0),
            Pos::new(0, 1),
            Pos::new(0, -1),
        ] {
            let new_pos = curr + dir;
            match grid[new_pos.y as usize][new_pos.x as usize] {
                GridPos::Wall => continue,
                GridPos::End | GridPos::Empty => {
                    if !dists.contains_key(&new_pos) {
                        new_curr = Some(new_pos)
                    }
                }
            }
        }
        curr = new_curr.unwrap();
        curr_score += 1;
    }
    dists
        .iter()
        .map(|(curr, base_score)| {
            let mut count = 0;
            for dir in [
                Pos::new(2, 0),
                Pos::new(1, 1),
                Pos::new(0, 2),
                Pos::new(-1, 1),
                Pos::new(-2, 0),
                Pos::new(-1, -1),
                Pos::new(0, -2),
                Pos::new(1, -1),
            ] {
                let new_pos = *curr + dir;
                if new_pos.x < 0
                    || new_pos.y < 0
                    || new_pos.x == grid.len() as isize
                    || new_pos.y == grid[0].len() as isize
                {
                    continue;
                }
                match grid[new_pos.y as usize][new_pos.x as usize] {
                    GridPos::Wall => continue,
                    GridPos::End | GridPos::Empty => {
                        if let Some(dest_score) = dists.get(&new_pos) {
                            if *dest_score >= *base_score + 2 + M {
                                count += 1
                            }
                        }
                    }
                }
            }
            count
        })
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
enum GridPos {
    Wall,
    End,
    Empty,
}

impl GridPos {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'E' => Self::End,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Reindeer {
    pos: Pos,
    has_cheated: bool,
    score: usize,
}

impl PartialEq for Reindeer {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.score == other.score && self.has_cheated == other.has_cheated
    }
}
impl Eq for Reindeer {}

impl PartialOrd for Reindeer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Reindeer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.score).cmp(&(other.score))
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

    pub fn dist(&self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
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
        assert_eq!(
            solve::<2>(include_str!("../../example")),
            14 + 14 + 2 + 4 + 2 + 3 + 1 + 1 + 1 + 1 + 1
        );
    }
}
