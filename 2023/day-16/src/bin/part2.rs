use std::cmp;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut max = 0;
    for i in 0..4 {
        let iters = match i % 2 {
            0 => input.len(),
            1 => input[0].len(),
            _ => unreachable!(),
        };
        for j in 0..iters {
            let mut visited: Vec<((usize, usize), usize)> = Vec::new();
            let mut curriter: Vec<((usize, usize), usize)> = Vec::new();
            match i {
                0 => {
                    curriter.push(((j, 0), 0));
                    visited.push(((j, 0), 0));
                }
                1 => {
                    curriter.push(((0, j), 1));
                    visited.push(((0, j), 1));
                }
                2 => {
                    curriter.push(((j, input[0].len() - 1), 2));
                    visited.push(((j, input[0].len() - 1), 2));
                }
                3 => {
                    curriter.push(((input.len() - 1, j), 3));
                    visited.push(((input.len() - 1, j), 3));
                }
                _ => (),
            }
            loop {
                let mut nextiter: Vec<((usize, usize), usize)> = Vec::new();
                for t in curriter.clone() {
                    // println!("{} {:?}", input[t.0 .0][t.0 .1], ((t.0.0+1, t.0.1+1), t.1));
                    match t.1 {
                        0 => match input[t.0 .0][t.0 .1] {
                            '.' | '-' => {
                                if t.0 .1 != input[0].len() - 1 {
                                    if !visited.contains(&((t.0 .0, t.0 .1 + 1), 0)) {
                                        nextiter.push(((t.0 .0, t.0 .1 + 1), 0));
                                    }
                                }
                            }
                            '|' => {
                                if t.0 .0 != input.len() - 1 {
                                    if !visited.contains(&((t.0 .0 + 1, t.0 .1), 1)) {
                                        nextiter.push(((t.0 .0 + 1, t.0 .1), 1));
                                    }
                                }
                                if t.0 .0 != 0 {
                                    if !visited.contains(&((t.0 .0 - 1, t.0 .1), 3)) {
                                        nextiter.push(((t.0 .0 - 1, t.0 .1), 3));
                                    }
                                }
                            }
                            '/' => {
                                if t.0 .0 != 0 {
                                    if !visited.contains(&((t.0 .0 - 1, t.0 .1), 3)) {
                                        nextiter.push(((t.0 .0 - 1, t.0 .1), 3));
                                    }
                                }
                            }
                            '\\' => {
                                if t.0 .0 != input.len() - 1 {
                                    if !visited.contains(&((t.0 .0 + 1, t.0 .1), 1)) {
                                        nextiter.push(((t.0 .0 + 1, t.0 .1), 1));
                                    }
                                }
                            }
                            _ => {
                                unreachable!();
                            }
                        },
                        1 => match input[t.0 .0][t.0 .1] {
                            '.' | '|' => {
                                if t.0 .0 != input.len() - 1 {
                                    if !visited.contains(&((t.0 .0 + 1, t.0 .1), 1)) {
                                        nextiter.push(((t.0 .0 + 1, t.0 .1), 1));
                                    }
                                }
                            }
                            '-' => {
                                if t.0 .1 != input[0].len() - 1 {
                                    if !visited.contains(&((t.0 .0, t.0 .1 + 1), 0)) {
                                        nextiter.push(((t.0 .0, t.0 .1 + 1), 0));
                                    }
                                }
                                if t.0 .1 != 0 {
                                    if !visited.contains(&((t.0 .0, t.0 .1 - 1), 2)) {
                                        nextiter.push(((t.0 .0, t.0 .1 - 1), 2));
                                    }
                                }
                            }
                            '/' => {
                                if t.0 .1 != 0 {
                                    if !visited.contains(&((t.0 .0, t.0 .1 - 1), 2)) {
                                        nextiter.push(((t.0 .0, t.0 .1 - 1), 2));
                                    }
                                }
                            }
                            '\\' => {
                                if t.0 .1 != input[0].len() - 1 {
                                    if !visited.contains(&((t.0 .0, t.0 .1 + 1), 0)) {
                                        nextiter.push(((t.0 .0, t.0 .1 + 1), 0));
                                    }
                                }
                            }
                            _ => {
                                unreachable!();
                            }
                        },
                        2 => match input[t.0 .0][t.0 .1] {
                            '.' | '-' => {
                                if t.0 .1 != 0 {
                                    if !visited.contains(&((t.0 .0, t.0 .1 - 1), 2)) {
                                        nextiter.push(((t.0 .0, t.0 .1 - 1), 2));
                                    }
                                }
                            }
                            '|' => {
                                if t.0 .0 != input[0].len() - 1 {
                                    if !visited.contains(&((t.0 .0 + 1, t.0 .1), 1)) {
                                        nextiter.push(((t.0 .0 + 1, t.0 .1), 1));
                                    }
                                }
                                if t.0 .0 != 0 {
                                    if !visited.contains(&((t.0 .0 - 1, t.0 .1), 3)) {
                                        nextiter.push(((t.0 .0 - 1, t.0 .1), 3));
                                    }
                                }
                            }
                            '/' => {
                                if t.0 .0 != input.len() - 1 {
                                    if !visited.contains(&((t.0 .0 + 1, t.0 .1), 1)) {
                                        nextiter.push(((t.0 .0 + 1, t.0 .1), 1));
                                    }
                                }
                            }
                            '\\' => {
                                if t.0 .0 != 0 {
                                    if !visited.contains(&((t.0 .0 - 1, t.0 .1), 3)) {
                                        nextiter.push(((t.0 .0 - 1, t.0 .1), 3));
                                    }
                                }
                            }
                            // '\\' => if !visited.contains(&((t.0 .0, t.0 .1), false)) {},
                            _ => {
                                unreachable!();
                            }
                        },
                        3 => match input[t.0 .0][t.0 .1] {
                            '.' | '|' => {
                                if t.0 .0 != 0 {
                                    if !visited.contains(&((t.0 .0 - 1, t.0 .1), 3)) {
                                        nextiter.push(((t.0 .0 - 1, t.0 .1), 3));
                                    }
                                }
                            }
                            '-' => {
                                if t.0 .1 != input[0].len() - 1 {
                                    if !visited.contains(&((t.0 .0, t.0 .1 + 1), 0)) {
                                        nextiter.push(((t.0 .0, t.0 .1 + 1), 0));
                                    }
                                }
                                if t.0 .1 != 0 {
                                    if !visited.contains(&((t.0 .0, t.0 .1 - 1), 2)) {
                                        nextiter.push(((t.0 .0, t.0 .1 - 1), 2));
                                    }
                                }
                            }
                            '/' => {
                                if t.0 .1 != input[0].len() - 1 {
                                    if !visited.contains(&((t.0 .0, t.0 .1 + 1), 0)) {
                                        nextiter.push(((t.0 .0, t.0 .1 + 1), 0));
                                    }
                                }
                            }
                            '\\' => {
                                if t.0 .1 != 0 {
                                    if !visited.contains(&((t.0 .0, t.0 .1 - 1), 2)) {
                                        nextiter.push(((t.0 .0, t.0 .1 - 1), 2));
                                    }
                                }
                            }
                            _ => {
                                unreachable!();
                            }
                        },
                        _ => todo!(),
                    }
                }
                for i in nextiter.clone() {
                    visited.push(i);
                }
                if nextiter.len() == 0 {
                    break;
                }
                curriter = nextiter;
            }
            {
                let ans = visited
                    .iter()
                    .map(|(a, _)| *a)
                    .collect::<HashSet<(usize, usize)>>()
                    .len();
                if ans > max {
                    max = ans;
                    println!("{max} {i} {j}")
                }
                // println!("{}", ans.len());
            }
        }
    }
    println!("{max}");
}
