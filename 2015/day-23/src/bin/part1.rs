use core::panic;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Val {
    Value(i8),
    Char(char),
}

#[derive(Debug)]
enum Line<'a> {
    One((&'a str, Val)),
    Two((&'a str, char, i8)),
}
impl<'a> Line<'a> {
    pub fn getfirst(&self) -> &'a str {
        match self {
            Line::One(x) => x.0,
            Line::Two(x) => x.0,
        }
    }
}
fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            if parts.len() == 2 {
                let second = if parts[0].chars().nth(0).unwrap() == 'j' {
                    Val::Value(parts[1].parse::<i8>().unwrap())
                } else {
                    Val::Char(parts[1].chars().nth(0).unwrap())
                };
                Line::One((parts[0], second))
            } else {
                Line::Two((
                    parts[0],
                    parts[1].chars().nth(0).unwrap(),
                    parts[2].parse::<i8>().unwrap(),
                ))
            }
        })
        .collect::<Vec<Line>>();

    let to_eval: Vec<usize> = input
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            if let Line::Two(x) = line {
                Some(((i + 1), ((i as i8 + x.2) as usize)))
            } else {
                None
            }
        })
        .fold(vec![0], |mut sum, curr| {
            sum.push(curr.0);
            sum.push(curr.1);
            sum
        });

    println!("{to_eval:?}");
    let trips = to_eval
        .iter()
        .map(|&index| {
            let mut i = index;
            let mut ops: Vec<&Line> = Vec::new();
            let end = 'outer: loop {
                if i >= input.len() {
                    break None;
                }
                while input[i].getfirst().chars().nth(0).unwrap() == 'j' {
                    if let Line::One((_, Val::Value(x))) = input[i] {
                        i = (i as i8 + x) as usize;
                    } else {
                        break;
                    }
                    if i >= input.len() {
                        break 'outer None;
                    }
                }
                if let Line::Two(_) = input[i] {
                    break Some(i);
                }
                ops.push(&input[i]);
                i += 1;
            };
            (index, (end, ops))
        })
        .collect::<HashMap<usize, (Option<usize>, Vec<&Line>)>>();
    for trip in &trips {
        println!("{trip:?}");
    }
    let mut vals = [0; 2];
    let mut pos = 0;
    loop {
        let (newpos, ops) = trips.get(&pos).unwrap();
        for op in ops {
            if let Line::One((command, Val::Char(register))) = op {
                match *command {
                    "inc" => vals[(*register as u8 - b'a') as usize] += 1,
                    "hlf" => vals[(*register as u8 - b'a') as usize] /= 2,
                    "tpl" => vals[(*register as u8 - b'a') as usize] *= 3,
                    _ => unreachable!(),
                }
            } else {
                panic!("Shouldn't happen: {op:?}");
            }
        }
        if let Some(x) = newpos {
            let line = &input[*x];
            match line {
                Line::Two(("jie", register, end)) => {
                    pos = if vals[(*register as u8 - b'a') as usize] % 2 == 0 {
                        (*x as i8 + end) as usize
                    } else {
                        *x + 1
                    }
                }
                Line::Two(("jio", register, end)) => {
                    pos = if vals[(*register as u8 - b'a') as usize]== 1 {
                        (*x as i8 + end) as usize
                    } else {
                        *x + 1
                    }
                }
                _ => unreachable!(),
            }
        } else {
            break;
        }
    }
    println!("{vals:?}");
}
