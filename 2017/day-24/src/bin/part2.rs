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
    let ans = solve(input, 0).0;
    println!("{ans}");
}

fn solve(maps: HashSet<(u8, u8)>, curr: u8) -> (u64, u64) {
    let mut max = (0, 0);
    for map in maps.iter().filter(|x| curr == x.0) {
        let mut copy = maps.clone();
        copy.remove(&map);
        let solved = solve(copy, map.1);
        let value =(solved.0 + map.0 as u64 + map.1 as u64, solved.1) ;
        if solved.1 > max.1 {
            max = value;
        } else if solved.1 == max.1 {
            max.0 = cmp::max(max.0, value.0);
        }
    }
    for map in maps.iter().filter(|x| curr == x.1 && curr != x.0) {
        let mut copy = maps.clone();
        copy.remove(&map);
        let solved = solve(copy, map.0);
        let value =(solved.0 + map.0 as u64 + map.1 as u64, solved.1) ;
        if solved.1 > max.1 {
            max = value;
        } else if solved.1 == max.1 {
            max.0 = cmp::max(max.0, value.0);
        }
    }
    (max.0, max.1 + 1)
}
