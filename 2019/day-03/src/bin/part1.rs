use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Movement {
    R(usize),
    L(usize),
    U(usize),
    D(usize),
}

fn main() {
    let input = include_str!("../../input").lines().map(|line| {
        line.split(',')
            .map(|x| {
                let value = x
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                match x.chars().next().unwrap() {
                    'R' => Movement::R(value),
                    'L' => Movement::L(value),
                    'U' => Movement::U(value),
                    'D' => Movement::D(value),
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<Movement>>()
    });
    let wires = input
        .map(|line| {
            let mut curr = Point { x: 0, y: 0 };
            let mut ret = HashSet::new();
            for movement in line {
                match movement {
                    Movement::R(x) => {
                        for _ in 0..x {
                            curr.x += 1;
                            ret.insert(curr.clone());
                        }
                    }
                    Movement::L(x) => {
                        for _ in 0..x {
                            curr.x -= 1;
                            ret.insert(curr.clone());
                        }
                    }
                    Movement::U(x) => {
                        for _ in 0..x {
                            curr.y += 1;
                            ret.insert(curr.clone());
                        }
                    }
                    Movement::D(x) => {
                        for _ in 0..x {
                            curr.y -= 1;
                            ret.insert(curr.clone());
                        }
                    }
                }
            }
            ret
        })
        .collect::<Vec<HashSet<Point>>>();
    // println!("{:?}", wires[0]);
    // println!("{:?}", wires[1]);
    let min = wires[0]
        .iter()
        .filter(|x| wires[1].contains(x))
        .inspect(|x| println!("{x:?}"))
        .map(|x| x.x.abs() + x.y.abs())
        .min()
        .unwrap();
    println!("{min}");
}
