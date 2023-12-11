use std::{cmp::Ordering, collections::HashSet};
fn main() {
    let input = include_str!("../../input.txt");
    let mut seeds = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|string| string.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|a| (a[0], a[0] + a[1]))
        .collect::<HashSet<(u64, u64)>>();

    let maps = input
        .split("\r\n\r\n")
        .skip(1)
        .map(|lines| {
            let mut map = lines
                .lines()
                .skip(1)
                .filter(|line| line.trim().len() != 0)
                .map(|line| {
                    line.split(" ")
                        .take(3)
                        .map(|value| value.trim().parse::<u64>().unwrap())
                        .collect::<Vec<u64>>()
                })
                .map(|line| (line[0], line[1], line[2]))
                .collect::<Vec<(u64, u64, u64)>>();
            map.sort_by(|value, other| match value.1 < other.1 {
                true => Ordering::Less,
                false => Ordering::Greater,
            });
            map
        })
        .collect::<Vec<Vec<(u64, u64, u64)>>>();

    for map in maps {
        let mut newseeds = HashSet::<(u64, u64)>::new();
        'exit: for (mut start, end) in seeds {
            for (a, b, c) in &map {
                let (a, b, c) = (*a, *b, *c);
                if start >= b && (start < (b + c)) {
                    if end < (b + c) {
                        newseeds.insert((start + a - b, end + a - b));
                        continue 'exit;
                    } else {
                        newseeds.insert((start + a - b, c + a));
                        start = b + c;
                    }
                }
            }
            newseeds.insert((start, end));
        }
        seeds = newseeds;
    }
    println!("{}", seeds.len());
    let min = seeds.iter().map(|x| x.0).min().unwrap();
    println!("{min}");
}
