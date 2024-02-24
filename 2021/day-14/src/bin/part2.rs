use std::collections::HashMap;

fn main() {
    let mut input = include_str!("../../input").split("\n\n");
    let polymer = input.next().unwrap().chars().collect::<Vec<char>>();
    let transformations = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split(" -> ");
            let left = split.next().unwrap().chars().take(2).collect::<Vec<char>>();
            (
                (left[0], left[1]),
                split.next().unwrap().chars().next().unwrap(),
            )
        })
        .collect::<HashMap<(char, char), char>>();
    let mut counts = transformations
        .iter()
        .map(|x| (*x.0, 0))
        .collect::<HashMap<(char, char), u64>>();

    for p in polymer.windows(2) {
        *counts.get_mut(&(p[0], p[1])).unwrap() += 1;
    }

    for _ in 0..40 {
        counts = counts
            .iter()
            .map(|(k, v)| (k.0, transformations.get(k).unwrap(), k.1, v))
            .fold(HashMap::new(), |mut sum, curr| {
                if let Some(x) = sum.get_mut(&(curr.0, *curr.1)) {
                    *x += curr.3;
                } else {
                    sum.insert((curr.0, *curr.1), *curr.3);
                }
                if let Some(x) = sum.get_mut(&(*curr.1, curr.2)) {
                    *x += curr.3;
                } else {
                    sum.insert((*curr.1, curr.2), *curr.3);
                }
                sum
            });
    }
    let mut counts = counts.iter().fold([0; 26], |mut sum, curr| {
        println!("{curr:?}");
        sum[curr.0 .0 as usize - 65] += *curr.1;
        sum[curr.0 .1 as usize - 65] += *curr.1;
        sum
    });
    counts[polymer[0] as usize - 65] += 1;
    counts[polymer[polymer.len() - 1] as usize - 65] += 1;
    for x in counts.iter_mut() {
        *x /= 2;
    }
    println!("{counts:?}");
    println!("{}", {
        counts.iter().max().unwrap() - counts.iter().filter(|x| **x != 0).min().unwrap()
    })
}

