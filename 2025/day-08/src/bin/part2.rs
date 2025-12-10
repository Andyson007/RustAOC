use std::collections::{BTreeSet, HashSet};

fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

fn solve(input: &str) -> usize {
    let points = input.lines().map(Point::parse).collect::<Vec<_>>();
    let mut pairs = BTreeSet::new();
    for (pos, point) in points.iter().enumerate() {
        for other in &points[pos + 1..] {
            pairs.insert(Pair {
                a: *point,
                b: *other,
            });
        }
    }

    let mut graphs = points
        .into_iter()
        .map(|x| Graph {
            nodes: HashSet::from([x]),
        })
        .collect::<Vec<_>>();

    for pair in pairs {
        let second_pos = graphs.iter().position(|x| x.contains(&pair.b)).unwrap();
        let second = graphs.swap_remove(second_pos);
        if second.contains(&pair.a) {
            graphs.push(second);
            continue;
        }
        let first = graphs.iter_mut().find(|x| x.contains(&pair.a)).unwrap();
        first.merge(&second);

        if graphs.len() == 1 {
            return pair.a.x * pair.b.x;
        }
    }
    unreachable!()
}

#[derive(Debug)]
struct Graph {
    nodes: HashSet<Point>,
}

impl Graph {
    pub fn contains(&self, point: &Point) -> bool {
        self.nodes.contains(point)
    }

    pub fn merge(&mut self, other: &Graph) {
        self.nodes = self
            .nodes
            .union(&other.nodes)
            .copied()
            .collect::<HashSet<_>>();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair {
    a: Point,
    b: Point,
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.a.dist_squared(&self.b)).cmp(&other.a.dist_squared(&other.b))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    pub fn dist_squared(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) * self.x.abs_diff(other.x)
            + self.y.abs_diff(other.y) * self.y.abs_diff(other.y)
            + self.z.abs_diff(other.z) * self.z.abs_diff(other.z)
    }

    pub fn parse(point: &str) -> Self {
        let mut iter = point.split(',').map(|x| x.parse().unwrap());
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        Self { x, y, z }
    }
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let ans = solve(
            r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689",
        );
        assert_eq!(ans, 216 * 117);
    }
}
