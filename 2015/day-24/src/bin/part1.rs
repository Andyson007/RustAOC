use std::collections::HashSet;

use itertools::{Combinations, Itertools};

fn main() {
    // let input = "1\n2\n3\n4\n5\n7\n8\n9\n10\n11"
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<HashSet<u64>>();
    let third = input.iter().sum::<u64>() / 3;
    for size in 1..input.len() {
        for combination in input.iter().combinations(size).filter(|x| {
            if x.iter().map(|x| **x).sum::<u64>() == third {
                true
            } else {
                false
            }
        }) {
            println!("combination: {combination:?}");
            for size in 1..input.len() {
                let filtered = input
                    .iter()
                    .combinations(size)
                    .filter(|x| {
                        for val in x {
                            if combination.contains(val) {
                                // println!("{x:?}");
                                return false;
                            }
                        }
                        true
                    })
                    .collect::<HashSet<Vec<&u64>>>();

                if filtered.iter().any(|x| {
                    if x.iter().map(|x| *x).sum::<u64>() == third {
                        true
                    } else {
                        false
                    }
                }) {
                    println!("{}", combination.iter().fold(1, |sum, curr| sum * **curr));
                    return;
                }
                if filtered.iter().all(|x| {
                    if x.iter().map(|x| *x).sum::<u64>() > third {
                        true
                    } else {
                        false
                    }
                }) {
                    break;
                }
            }
        }
    }
}
