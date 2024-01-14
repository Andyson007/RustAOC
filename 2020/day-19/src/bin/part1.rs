use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Rule {
    Const(char),
    Other(HashSet<Vec<u64>>),
}

fn main() {
    let input = include_str!("../../input.txt");
    let parts = input.split("\r\n\r\n").collect::<Vec<&str>>();
    let maps = parts[0]
        .lines()
        .map(|line| {
            let split = line.split(": ").collect::<Vec<&str>>();
            let id = split[0].parse::<u64>().unwrap();
            if split[1].chars().nth(0).unwrap() == '"' {
                (id, Rule::Const(split[1].chars().nth(1).unwrap()))
            } else {
                let rules = split[1]
                    .split(" | ")
                    .map(|s| {
                        s.split_whitespace()
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>()
                    })
                    .collect::<HashSet<Vec<u64>>>();
                (id, Rule::Other(rules))
            }
        })
        .collect::<HashMap<u64, Rule>>();
    let possible = solve(0, &maps);
    let ans = parts[1]
        .lines()
        .filter(|x| possible.contains(&x.to_string()))
        .count();
    println!("{ans}");
}

fn solve(id: u64, maps: &HashMap<u64, Rule>) -> HashSet<String> {
    match maps.get(&id).unwrap() {
        Rule::Const(x) => HashSet::from([x.to_string()]),
        Rule::Other(x) => {
            let mut ret = HashSet::new();
            for v in x {
                let mut pushret = solve(v[0], &maps);
                for a in v.iter().skip(1) {
                    let mut new = HashSet::new();
                    for b in solve(*a, &maps) {
                        for val in pushret.clone() {
                            new.insert(val + b.as_str());
                        }
                    }
                    pushret = new;
                }
                for val in pushret {
                    ret.insert(val);
                }
            }
            ret
        }
    }
}
