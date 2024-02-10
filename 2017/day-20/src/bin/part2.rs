use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    pos: (i64, i64, i64),
    vel: (i64, i64, i64),
    acc: (i64, i64, i64),
}

fn main() {
    let mut input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let arr = line
                .split(", ")
                .map(|x| {
                    x.split(",")
                        .map(|s| {
                            s.chars()
                                .skip_while(|x| !x.is_digit(10) && *x != '-')
                                .take_while(|x| x.is_digit(10) || *x == '-')
                                .collect::<String>()
                                .parse::<i64>()
                                .unwrap()
                        })
                        .collect::<Vec<i64>>()
                })
                .collect::<Vec<Vec<i64>>>();
            Point {
                pos: (arr[0][0], arr[0][1], arr[0][2]),
                vel: (arr[1][0], arr[1][1], arr[1][2]),
                acc: (arr[2][0], arr[2][1], arr[2][2]),
            }
        })
        .collect::<Vec<Point>>();

    let mut count = 0;
    while !input.is_empty() {
        //remove all colliding particles
        let mut counts = HashMap::new();
        for line in &input {
            if let Some(x) = counts.get_mut(&line.pos) {
                *x += 1;
            } else {
                counts.insert(line.pos, 1);
            }
        }
        let todel = counts
            .iter()
            .filter_map(|(k, v)| if *v == 1 { None } else { Some(*k) })
            .collect::<HashSet<(i64, i64, i64)>>();

        input = input
            .iter()
            .filter(|x| !todel.contains(&x.pos))
            .map(|x| x.clone())
            .collect::<Vec<Point>>();

        // Remove all the asteroids that are the furthest away with the highest acceleration
        let dist = input
            .iter()
            .map(|x| x.pos.0.abs_diff(0) + x.pos.1.abs_diff(0) + x.pos.2.abs_diff(0))
            .max()
            .unwrap();
        let furthest = input
            .iter()
            .filter(|x| x.pos.0.abs_diff(0) + x.pos.1.abs_diff(0) + x.pos.2.abs_diff(0) == dist)
            .filter(|x| {
                (x.pos.0 * x.vel.0 >= 0 && x.pos.0 * x.acc.0 >= 0)
                    && (x.pos.1 * x.vel.1 >= 0 && x.pos.1 * x.acc.1 >= 0)
                    && (x.pos.2 * x.vel.2 >= 0 && x.pos.2 * x.acc.2 >= 0)
            })
            .map(|x| x.clone())
            .collect::<HashSet<Point>>();
        let accel = input
            .iter()
            .map(|x| x.acc.0.abs_diff(0) + x.acc.1.abs_diff(0) + x.acc.2.abs_diff(0))
            .max()
            .unwrap();
        let most = input
            .iter()
            .filter(|x| (x.acc.0.abs_diff(0) + x.acc.1.abs_diff(0) + x.acc.2.abs_diff(0)) == accel)
            .map(|x| x.clone())
            .collect::<HashSet<Point>>();
        let todel = most.intersection(&furthest).collect::<HashSet<&Point>>();
        count += todel.len();
        input = input
            .iter()
            .filter(|x| !todel.contains(x))
            .map(|x| x.clone())
            .collect::<Vec<Point>>();

        // Move all the particles
        for particle in input.iter_mut() {
            particle.vel.0 += particle.acc.0;
            particle.vel.1 += particle.acc.1;
            particle.vel.2 += particle.acc.2;

            particle.pos.0 += particle.vel.0;
            particle.pos.1 += particle.vel.1;
            particle.pos.2 += particle.vel.2;
        }
    }
    println!("{}", count)
}
