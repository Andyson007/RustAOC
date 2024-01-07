use std::collections::{HashMap, HashSet};

fn main() {
    let mut input = include_str!("../../input.txt").split("\r\n\r\n");
    let mappings = input
        .next()
        .unwrap()
        .lines()
        .rev()
        .map(|line| {
            let line = line.split(" => ").collect::<Vec<&str>>();
            (line[0], line[1])
        })
        .fold(HashMap::<&str, Vec<&str>>::new(), |mut sum, curr| {
            if let Some(x) = sum.get_mut(&curr.0) {
                x.push(curr.1);
            } else {
                sum.insert(curr.0, vec![curr.1]);
            }
            sum
        });

    let molecule = input.next().unwrap().lines().nth(0).unwrap();
    let mut molecules: HashSet<String> = HashSet::new();
    for map in mappings {
        for result in map.1 {
            let split = molecule.split(map.0).collect::<Vec<&str>>();
            for i in 1..split.len() {
                let mut new = split
                    .iter()
                    .take(i)
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(map.0);
                new.push_str(result);
                new.push_str(
                    split
                        .iter()
                        .skip(i)
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(map.0)
                        .as_str(),
                );
                molecules.insert(new.clone());
            }
        }
    }
    println!("{}", molecules.len());
}
