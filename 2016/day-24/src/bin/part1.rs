use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let maze = Maze::parse(include_str!("../../input"));
    let edges = maze.gen_edges();
    let ans = maze
        .destinations
        .iter()
        .permutations(maze.destinations.len())
        .map(|x| {
                [maze.startpos]
                    .iter()
                    .chain(x)
                    .tuple_windows()
                    .map(|x: (&Pos, &Pos)| edges.get(&(*x.0, *x.1)).unwrap())
                    .sum::<usize>()
        })
        .min()
        .unwrap();
    println!("{:?}", ans);
}

#[derive(Debug, Eq, PartialEq)]
enum MazeTile {
    Empty,
    Wall,
    Destination,
    Start,
}

#[derive(Debug)]
struct Maze {
    maze: Vec<Vec<MazeTile>>,
    destinations: Vec<Pos>,
    startpos: Pos,
}

impl Maze {
    pub fn parse(data: &str) -> Self {
        let (maze, destinations, startpos) = data
            .lines()
            .enumerate()
            .map(|(linenr, line)| {
                let tiles = line
                    .chars()
                    .map(|c| match c {
                        '#' => MazeTile::Wall,
                        '.' => MazeTile::Empty,
                        '0' => MazeTile::Start,
                        _ => MazeTile::Destination,
                    })
                    .collect::<Vec<_>>();
                let destinations = tiles
                    .iter()
                    .enumerate()
                    .filter(|(_, d)| **d == MazeTile::Destination)
                    .map(|(i, _)| Pos { x: i, y: linenr })
                    .collect::<Vec<_>>();
                let startpos = tiles
                    .iter()
                    .enumerate()
                    .find(|x| *x.1 == MazeTile::Start)
                    .map(|x| Pos { x: x.0, y: linenr });
                (tiles, destinations, startpos)
            })
            .fold((Vec::new(), Vec::new(), None), |mut sum, curr| {
                sum.0.push(curr.0);
                sum.1.push(curr.1);
                if let Some(x) = curr.2 {
                    sum.2 = Some(x);
                }
                sum
            });
        Self {
            maze,
            destinations: destinations.into_iter().flatten().collect::<Vec<_>>(),
            startpos: startpos.unwrap(),
        }
    }

    pub fn gen_edges(&self) -> HashMap<(Pos, Pos), usize> {
        let mut ret = HashMap::new();
        'outer: for (&start, &end) in self
            .destinations
            .iter()
            .chain([&self.startpos])
            .tuple_combinations()
        {
            let mut next = BinaryHeap::new();

            next.push(Node {
                dist: 0,
                heuristic: start.dist(&end),
                pos: start,
            });
            let mut visited = HashSet::new();
            while let Some(x) = next.pop() {
                if x.pos == end {
                    ret.insert((start, end), x.dist);
                    ret.insert((end, start), x.dist);
                    continue 'outer;
                }
                for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let newpos = x.pos + dir;
                    if self[newpos] != MazeTile::Wall && !visited.contains(&newpos) {
                        next.push(Node {
                            dist: x.dist + 1,
                            heuristic: newpos.dist(&end),
                            pos: newpos,
                        });
                        visited.insert(newpos);
                    }
                }
            }
            panic!("Path wasn't found between two points");
        }
        ret
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    /// Distance travelled so far
    dist: usize,
    /// Heuristic of distance to the finish
    heuristic: usize,
    /// current location
    pos: Pos,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.dist + self.heuristic)
            .cmp(&(other.dist + self.heuristic))
            .reverse()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn dist(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl std::ops::Add<(i32, i32)> for Pos {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        let newx = self.x as i32 + rhs.0;
        let newy = self.y as i32 + rhs.1;
        Self {
            x: newx as usize,
            y: newy as usize,
        }
    }
}

impl From<(i32, i32)> for Pos {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0 as usize,
            y: value.1 as usize,
        }
    }
}

impl std::ops::Index<Pos> for Maze {
    type Output = MazeTile;

    fn index(&self, index: Pos) -> &Self::Output {
        &self.maze[index.y][index.x]
    }
}
