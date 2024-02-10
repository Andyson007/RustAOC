use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let split = line
                .split(", ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (split[0], split[1])
        })
        .collect::<HashSet<(usize, usize)>>();
    let mut areas = input
        .iter()
        .map(|x| (*x, Some(0)))
        .collect::<HashMap<(usize, usize), Option<u64>>>();
    let (istart, iend) = (
        input.iter().map(|x| x.0).min().unwrap(),
        input.iter().map(|x| x.0).max().unwrap(),
    );
    let (jstart, jend) = (
        input.iter().map(|x| x.1).min().unwrap(),
        input.iter().map(|x| x.1).max().unwrap(),
    );
    for i in istart..=iend {
        for j in jstart..=jend {
            if let Some(x) = closest((i, j), &input) {
                if i == istart || i == iend || j == jstart || j == jend {
                    *areas.get_mut(&x).unwrap() = None;
                } else {
                    match areas.get_mut(&x).unwrap() {
                        Some(x) => *x += 1,
                        None => (),
                    }
                }
            }
        }
    }
    println!("{areas:?}");
    let ans = areas.iter().filter_map(|(_k, v)| v.clone()).max().unwrap();
    println!("{ans}");
}

fn closest(curr: (usize, usize), input: &HashSet<(usize, usize)>) -> Option<(usize, usize)> {
    let mindist = input
        .iter()
        .map(|x| x.0.abs_diff(curr.0) + x.1.abs_diff(curr.1))
        .min()
        .unwrap();
    let closest = input
        .iter()
        .filter(|x| x.0.abs_diff(curr.0) + x.1.abs_diff(curr.1) == mindist)
        .map(|x| *x)
        .collect::<Vec<(usize, usize)>>();
    if closest.len()!=1 {
        None
    } else {
        Some(closest[0])
    }
}
