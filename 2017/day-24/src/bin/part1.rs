use std::{cmp, collections::HashSet};

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let split = line
                .split("/")
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            (split[0], split[1])
        })
        .collect::<HashSet<(u8, u8)>>();
    let ans = solve(input, 0);
    println!("{ans}");
}

fn solve(maps: HashSet<(u8, u8)>, curr: u8) -> u64 {
    let mut max = 0;
    for map in maps.iter().filter(|x| curr == x.0) {
        let mut copy = maps.clone();
        copy.remove(&map);
        max = cmp::max(max, map.1 as u64 + map.0 as u64 + solve(copy, map.1));
    }
    for map in maps.iter().filter(|x| curr == x.1 && curr != x.0) {
        let mut copy = maps.clone();
        copy.remove(&map);
        max = cmp::max(max, map.0 as u64 + map.1 as u64 + solve(copy, map.0));
    }
    max
}
