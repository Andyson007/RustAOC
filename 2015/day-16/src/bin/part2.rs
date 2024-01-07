use regex::Regex;
use std::collections::HashMap;

fn main() {
    let knowledge = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
    let input = include_str!("../../input.txt")
        .lines()
        .enumerate()
        .map(|(i, line)| {
            (
                i + 1,
                Regex::new(r"[:,] ")
                    .unwrap()
                    .split(line)
                    .skip(1)
                    .collect::<Vec<&str>>()
                    .chunks(2)
                    .map(|arr| (arr[0], arr[1].parse::<u8>().unwrap()))
                    .collect::<HashMap<&str, u8>>(),
            )
        })
        .collect::<HashMap<usize, HashMap<&str, u8>>>();
    for i in input.iter().filter(|sue| {
        for (k, v) in sue.1 {
            match *k {
                "cats" | "trees" => {
                    if knowledge.get(k).unwrap() >= v {
                        return false;
                    }
                }
                "pomeranians" | "goldfish" => {
                    if knowledge.get(k).unwrap() <= v {
                        return false;
                    }
                }
                _ => {
                    if knowledge.get(k).unwrap() != v {
                        return false;
                    }
                }
            }
        }
        true
    }) {
        println!("{i:?}");
    }
}
