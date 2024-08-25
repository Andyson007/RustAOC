use std::{
    collections::{BinaryHeap, HashSet},
    ops::Add,
};

fn main() {
    let input = 1352;
    let f = |pos: Pos| {
        (pos.x * pos.x + 3 * pos.x + 2 * pos.x * pos.y + pos.y + pos.y * pos.y + input).count_ones()
            & 1
            == 0
    };
    let mut binary_heap = BinaryHeap::new();
    let end = Pos { x: 31, y: 39 };
    let start = Pos { x: 1, y: 1 };
    binary_heap.push(Node {
        dist: 0,
        cost: start.dist(&end),
        location: start,
    });

    let mut visited = HashSet::from([start]);

    'outer: while let Some(x) = binary_heap.pop() {
        for dir in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
            let newpos = x.location + dir;
            if newpos == end {
                println!("{}", x.dist + 1);
                break 'outer;
            }
            if valid(newpos) && f(newpos) && !visited.contains(&newpos) {
                binary_heap.push(Node {
                    dist: x.dist + 1,
                    cost: newpos.dist(&end),
                    location: newpos,
                });
                visited.insert(newpos);
            };
        }
    }
}

fn valid(pos: Pos) -> bool {
    pos.x > 0 && pos.y > 0
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Node {
    dist: usize,
    cost: usize,
    location: Pos,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn dist(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Add<(isize, isize)> for Pos {
    type Output = Self;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}
