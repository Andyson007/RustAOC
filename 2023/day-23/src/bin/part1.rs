use std::{
    cmp,
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
enum Dir {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq)]
enum Spot {
    Path,
    Forest,
    Slope(Dir),
}

fn main() -> Result<(), ()> {
    let mut input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Spot::Path,
                    '#' => Spot::Forest,
                    '^' => Spot::Slope(Dir::North),
                    'v' => Spot::Slope(Dir::South),
                    '>' => Spot::Slope(Dir::East),
                    '<' => Spot::Slope(Dir::West),
                    _ => unreachable!(),
                })
                .collect::<Vec<Spot>>()
        })
        .collect::<Vec<Vec<Spot>>>();
    let start = Point {
        x: match input.first().unwrap().iter().position(|x| *x == Spot::Path) {
            Some(x) => x,
            None => return Err(()),
        },
        y: 1,
    };
    let end = Point {
        x: match input.last().unwrap().iter().position(|x| *x == Spot::Path) {
            Some(x) => x,
            None => return Err(()),
        },
        y: input.len() - 2,
    };
    input[0][start.x] = Spot::Forest;
    input[end.y + 1][end.x] = Spot::Forest;
    let mut crossings = HashSet::new();

    for i in 1..input.len() - 1 {
        for j in 1..input[i].len() - 1 {
            if input[i][j] == Spot::Forest {
                continue;
            }
            let mut count = 0;
            if input[i][j + 1] != Spot::Forest {
                count += 1;
            }
            if input[i][j - 1] != Spot::Forest {
                count += 1;
            }
            if input[i + 1][j] != Spot::Forest {
                count += 1;
            }
            if input[i - 1][j] != Spot::Forest {
                count += 1;
            }
            if count > 2 {
                crossings.insert(Point { x: j, y: i });
            }
        }
    }

    crossings.insert(start);
    crossings.insert(end);

    let mut nodes: HashMap<Point, HashSet<(usize, Point)>> = HashMap::new();
    {
        let mut considered = HashSet::new();
        let mut to_concider = Vec::new();
        let mut curr = start;

        loop {
            considered.insert(curr);
            let nexts = travel(&curr, &input, &crossings);
            nodes.insert(curr, nexts.clone());
            for (_, val) in nexts {
                if !considered.contains(&val) {
                    to_concider.push(val);
                }
            }

            if let Some(x) = to_concider.pop() {
                curr = x;
            } else {
                break;
            }
        }
    }

    let ans = solve(start, end, &nodes, HashSet::new()).unwrap();

    println!("{}", ans + 2);
    Ok(())
}

fn solve(
    start: Point,
    end: Point,
    nodes: &HashMap<Point, HashSet<(usize, Point)>>,
    mut visited: HashSet<Point>,
) -> Option<usize> {
    if start == end {
        Some(0)
    } else if visited.contains(&start) {
        None
    } else {
        visited.insert(start);
        let mut max = None;
        for next in nodes.get(&start).unwrap() {
            if !visited.contains(&next.1) {
                let score = solve(next.1, end, nodes, visited.clone());
                if let Some(x) = score {
                    let score = x + next.0;
                    // println!("{start:?}, {score}",);
                    if let Some(x) = max {
                        max = Some(cmp::max(x, score));
                    } else {
                        max = Some(score);
                    }
                }
            }
        }
        max
    }
}

fn travel(
    curr: &Point,
    input: &[Vec<Spot>],
    crossings: &HashSet<Point>,
) -> HashSet<(usize, Point)> {
    if input[curr.y][curr.x] == Spot::Forest {
        return HashSet::new();
    }
    let mut ret = HashSet::new();
    let mut to_concider = HashSet::new();
    if ![Spot::Forest, Spot::Slope(Dir::North)].contains(&input[curr.y + 1][curr.x]) {
        to_concider.insert(Point {
            x: curr.x,
            y: curr.y + 1,
        });
    }
    if ![Spot::Forest, Spot::Slope(Dir::South)].contains(&input[curr.y - 1][curr.x]) {
        to_concider.insert(Point {
            x: curr.x,
            y: curr.y - 1,
        });
    }
    if ![Spot::Forest, Spot::Slope(Dir::West)].contains(&input[curr.y][curr.x + 1]) {
        to_concider.insert(Point {
            x: curr.x + 1,
            y: curr.y,
        });
    }
    if ![Spot::Forest, Spot::Slope(Dir::East)].contains(&input[curr.y][curr.x - 1]) {
        to_concider.insert(Point {
            x: curr.x - 1,
            y: curr.y,
        });
    }

    for mut next in to_concider {
        let mut prev = *curr;
        for len in 1.. {
            if crossings.contains(&next) {
                ret.insert((len, next));
                break;
            }
            let tmp = Point {
                x: next.x + 1,
                y: next.y,
            };
            if ![Spot::Forest, Spot::Slope(Dir::North)].contains(&input[tmp.y][tmp.x])
                && prev != tmp
            {
                prev = next;
                next = tmp;
                continue;
            }
            let tmp = Point {
                x: next.x - 1,
                y: next.y,
            };
            if ![Spot::Forest, Spot::Slope(Dir::South)].contains(&input[tmp.y][tmp.x])
                && prev != tmp
            {
                prev = next;
                next = tmp;
                continue;
            }
            let tmp = Point {
                x: next.x,
                y: next.y + 1,
            };
            if ![Spot::Forest, Spot::Slope(Dir::West)].contains(&input[tmp.y][tmp.x]) && prev != tmp
            {
                prev = next;
                next = tmp;
                continue;
            }
            let tmp = Point {
                x: next.x,
                y: next.y - 1,
            };
            if ![Spot::Forest, Spot::Slope(Dir::East)].contains(&input[tmp.y][tmp.x]) && prev != tmp
            {
                prev = next;
                next = tmp;
                continue;
            }
            break;
        }
    }
    ret
}
