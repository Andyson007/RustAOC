use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Val<'a> {
    Value(u16),
    Other(&'a str),
}

#[derive(Debug, Clone, Copy)]
struct Five<'a> {
    left: Val<'a>,
    right: Val<'a>,
    op: Op,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
}

#[derive(Debug, Clone, Copy)]
enum Line<'a> {
    Five(Five<'a>),
    Four(&'a str),
    Three(&'a str),
}

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            (*split.last().unwrap(), (split))
        })
        .collect::<HashMap<&str, Vec<&str>>>();
    let mut known = input
        .iter()
        .filter(|(_k, v)| v.len() == 3)
        .filter(|(_k, v)| v[0].chars().nth(0).unwrap().is_digit(10))
        .map(|(k, v)| (*k, v[0].parse::<u16>().unwrap()))
        .collect::<HashMap<&str, u16>>();
    let mut input = input
        .iter()
        .filter(|(_k, v)| !((v.len() == 3) && v[0].chars().nth(0).unwrap().is_digit(10)))
        .map(|(k, v)| {
            (
                *k,
                match v.len() {
                    5 => {
                        let left = if let Ok(x) = v[0].parse::<u16>() {
                            Val::Value(x)
                        } else {
                            Val::Other(v[0])
                        };
                        let right = if let Ok(x) = v[2].parse::<u16>() {
                            Val::Value(x)
                        } else {
                            Val::Other(v[2])
                        };
                        Line::Five(Five {
                            left,
                            right,
                            op: match v[1] {
                                "OR" => Op::OR,
                                "AND" => Op::AND,
                                "LSHIFT" => Op::LSHIFT,
                                "RSHIFT" => Op::RSHIFT,
                                _ => unreachable!(),
                            },
                        })
                    }
                    4 => Line::Four(v[1]),
                    3 => Line::Three(v[0]),
                    _ => unreachable!(),
                },
            )
        })
        .collect::<HashMap<&str, Line>>();
    // while !known.contains_key("a") {
    while input.len() != 0 {
        // for line in &input {
        //     println!("{line:?}");
        // }
        // for k in &known {
        //     println!("{k:?}");
        // }
        // println!();
        for (k, v) in input.clone() {
            match v {
                Line::Five(x) => {
                    let left = match x.left {
                        Val::Value(x) => Some(x),
                        Val::Other(x) => known.get(&x).copied(),
                    };
                    let right = match x.right {
                        Val::Value(x) => Some(x),
                        Val::Other(x) => known.get(&x).copied(),
                    };
                    if let (Some(left), Some(right)) = (left, right) {
                        known.insert(
                            k,
                            match x.op {
                                Op::AND => left & right,
                                Op::OR => left | right,
                                Op::LSHIFT => left << right,
                                Op::RSHIFT => left >> right,
                            },
                        );
                        input.remove(k);
                    }
                }
                Line::Four(x) => {
                    if let Some(x) = known.get(x) {
                        known.insert(k, !x);
                        input.remove(k);
                    }
                }
                Line::Three(x) => {
                    if let Some(x) = known.get(x) {
                        known.insert(k, *x);
                        input.remove(k);
                    }
                }
            }
        }
    }
    // for line in &input {
    //     println!("{line:?}");
    // }
    // for k in &known {
    //     println!("{k:?}");
    // }
    // println!();
    println!("{:?}", known.get("a"));
}
