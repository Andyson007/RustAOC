use std::cmp;

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

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
    let input = include_str!("../../input")
        .lines()
        .map(|line| {
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
        })
        .collect::<Vec<Vec<Movement>>>();
    let wires = input
        .iter()
        .map(|line| {
            let mut curr = Point { x: 0, y: 0 };
            let mut ret = Vec::new();
            for movement in line {
                match movement {
                    Movement::R(x) => {
                        for _ in 0..*x {
                            curr.x += 1;
                            ret.push(curr.clone());
                        }
                    }
                    Movement::L(x) => {
                        for _ in 0..*x {
                            curr.x -= 1;
                            ret.push(curr.clone());
                        }
                    }
                    Movement::U(x) => {
                        for _ in 0..*x {
                            curr.y += 1;
                            ret.push(curr.clone());
                        }
                    }
                    Movement::D(x) => {
                        for _ in 0..*x {
                            curr.y -= 1;
                            ret.push(curr.clone());
                        }
                    }
                }
            }
            ret
        })
        .collect::<Vec<Vec<Point>>>();
    // println!("{:?}", wires[0]);
    // println!("{:?}", wires[1]);
    let ans = wires[0]
        .iter()
        .enumerate()
        .filter(|x| wires[1].contains(x.1))
        .map(|(dist, point)| (wires[1].iter().position(|x| x == point).unwrap(), dist))
        .fold_while(usize::MAX, |sum, curr| {
            if curr.0 > sum {
                Done(sum)
            } else {
                Continue(cmp::min(curr.0 + curr.1, sum))
            }
        })
        .into_inner()
        + 2;
    println!("{ans}");
}
