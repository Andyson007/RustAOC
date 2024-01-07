use std::collections::HashSet;

use itertools::{Combinations, Itertools};

fn main() {
    // let input = "1\n2\n3\n4\n5\n7\n8\n9\n10\n11"
        let input = include_str!("../../input.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<HashSet<u64>>();
    let fourth = input.iter().sum::<u64>() / 4;
    for size in 1..input.len() {
        for opt in input
            .iter()
            .combinations(size)
            .filter(|x| x.iter().map(|x| **x).sum::<u64>() == fourth)
            .map(|x| (x.iter().fold(1, |sum, curr| sum * **curr), x))
            .sorted_by(|a, b| a.0.cmp(&b.0))
        {
            for size in size.. {
                for opt2 in input
                    .iter()
                    .combinations(size)
                    .filter(|x| x.iter().map(|x| **x).sum::<u64>() == fourth)
                    .filter(|x| {
                        for val in x {
                            if opt.1.contains(&val) {
                                return false;
                            }
                        }
                        true
                    })
                {
                    for size in size.. {
                        for opt3 in input
                            .iter()
                            .combinations(size)
                            .filter(|x| x.iter().map(|x| **x).sum::<u64>() == fourth)
                            .filter(|x| {
                                for val in x {
                                    if opt.1.contains(&val) || opt2.contains(&val) {
                                        return false;
                                    }
                                }
                                true
                            })
                        {
                            println!("{}", opt.0);
                            return;
                        }
                    }
                }
            }
        }
    }
}
