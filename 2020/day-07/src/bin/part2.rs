use std::collections::HashMap;

fn main() {
    let mut input = include_str!("../../input.txt")
        .lines()
        .map(|x| {
            let split = x.split_whitespace().collect::<Vec<&str>>();
            let curr = split[..2].join(" ");
            (
                curr,
                split[4..]
                    .join(" ")
                    .split(",")
                    .map(|x| {
                        let split = x
                            .split_whitespace()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>();
                        if split[0] == "no" {
                            None
                        } else {
                            Some((
                                split
                                    .iter()
                                    .skip(1)
                                    .take(2)
                                    .map(|x| x.clone())
                                    .collect::<Vec<String>>()
                                    .join(" ")
                                    .to_string(),
                                split[0].parse::<u64>().unwrap(),
                            ))
                        }
                    })
                    .collect::<Option<HashMap<String, u64>>>(),
            )
        })
        .collect::<HashMap<String, Option<HashMap<String, u64>>>>();
    let mut possible = HashMap::new();
    for (k, v) in input.clone() {
        if v.is_none() {
            possible.insert(k.clone(), 0);
            input.remove(&k);
        }
    }

    loop {
        let mut found = false;
        for (k, v) in input.clone() {
            if v.clone()
                .unwrap()
                .iter()
                .all(|(k, _v)| possible.contains_key(k))
            {
                possible.insert(
                    k.clone(),
                    v.clone()
                        .unwrap()
                        .iter()
                        .map(|(k, v)| (1 + possible.get(k).unwrap()) * v)
                        .sum(),
                );
                found = true;
                input.remove(&k);
            }
        }
        if !found {
            break;
        }
    }
    println!("{}", possible.get("shiny gold").unwrap());
}
