use std::collections::{HashMap, HashSet};

fn main() {
    let mut input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let split = line.split(" <-> ").collect::<Vec<&str>>();
            (
                split[0].parse::<usize>().unwrap(),
                split[1]
                    .split(", ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<HashSet<usize>>(),
            )
        })
        .collect::<HashMap<usize, HashSet<usize>>>();
    let mut count = 0;
    while input.len() != 0 {
        count += 1;
        let mut visited = HashSet::new();
        let mut consider = HashSet::from([*input.iter().nth(0).unwrap().0]);
        while consider.len() != 0 {
            for c in consider.clone() {
                consider.remove(&c);
                let vals = input.get(&c).unwrap();
                for v in vals {
                    if !visited.contains(v) {
                        consider.insert(*v);
                        visited.insert(*v);
                    }
                }
            }
        }
        for v in visited {
            input.remove(&v);
        }
    }
    println!("{count}");
}
