use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::convert::Infallible;
use std::ops::Neg;
use std::{
    ops::{Add, AddAssign, Mul, SubAssign},
    str::FromStr,
};

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
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

    let mut deer = BinaryHeap::from([Reverse(Reindeer {
        pos: start.unwrap(),
        dir: Pos::new(1, 0),
        visited: HashSet::new(),
        score: 0,
    })]);

    let mut visited = HashMap::new();
    let mut best_visited = HashSet::new();
    let mut best_score = None;
    while let Some(next) = deer.pop() {
        if let Some(score) = visited.get(&(next.0.pos, next.0.dir)) {
            if *score < next.0.score {
                continue;
            }
        }
        visited.insert((next.0.pos, next.0.dir), next.0.score);
        if grid[next.0.pos.y as usize][next.0.pos.x as usize] == GridPos::End {
            if best_score.is_none() {
                best_score = Some(next.0.score);
            } else if next.0.score > best_score.unwrap() {
                break;
            }
            for pos in &next.0.visited {
                best_visited.insert(*pos);
            }
        }

        for dir in [
            Pos::new(1, 0),
            Pos::new(-1, 0),
            Pos::new(0, 1),
            Pos::new(0, -1),
        ] {
            if dir == -next.0.dir {
                continue;
            }
            let new_pos = next.0.pos + dir;
            match grid[new_pos.y as usize][new_pos.x as usize] {
                GridPos::Wall => continue,
                GridPos::End | GridPos::Empty => {
                    if dir == next.0.dir {
                        deer.push(Reverse(Reindeer {
                            pos: new_pos,
                            dir,
                            visited: {
                                let mut visited = next.0.visited.clone();
                                visited.insert(next.0.pos);
                                visited
                            },
                            score: next.0.score + 1,
                        }))
                    } else {
                        deer.push(Reverse(Reindeer {
                            pos: new_pos,
                            dir,
                            visited: {
                                let mut visited = next.0.visited.clone();
                                visited.insert(next.0.pos);
                                visited
                            },
                            score: next.0.score + 1001,
                        }))
                    }
                }
            }
        }
    }

    best_visited.len() + 1
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

#[derive(Debug)]
struct Reindeer {
    pos: Pos,
    dir: Pos,
    visited: HashSet<Pos>,
    score: usize,
}

impl PartialEq for Reindeer {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.dir == other.pos && self.score == other.score
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
        self.score.cmp(&other.score)
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
    fn example_a() {
        assert_eq!(
            solve(
                "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            ),
            45
        )
    }

    #[test]
    fn example_b() {
        assert_eq!(
            solve(
                "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            ),
            64
        )
    }
}
