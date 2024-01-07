use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let name = line.split_whitespace().nth(0).unwrap();
            let val1 = line
                .split_whitespace()
                .nth(3)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let val2 = line
                .split_whitespace()
                .nth(6)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let val3 = line
                .split_whitespace()
                .nth(13)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            (name, (val1, val2, val3))
        })
        .collect::<Vec<(&str, (u32, u32, u32))>>();
    let distances = input
        .iter()
        .map(|(name, (speed, duration, rest))| {
            let mut total = 0;
            let mut second = 0;
            'outer: loop {
                for _ in 0..*duration {
                    if second == 2503 {
                        break 'outer;
                    }
                    total += speed;
                    second += 1;
                }
                if second + *rest >= 2503 {
                    break;
                }
                second += *rest;
            }
            (*name, total)
        })
        .collect::<HashMap<&str, u32>>();
    for (name, distance) in &distances {
        println!("{name}: {distance}");
    }
    println!();
    let max = distances.iter().map(|(_k, v)| v).max().unwrap();
    println!("max: {max}");
}
