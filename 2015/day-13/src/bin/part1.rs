use std::collections::HashMap;
use std::cmp;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let mut val = line.split_whitespace();
            (
                val.next().unwrap(),
                val.nth(1).unwrap() == "gain",
                val.next().unwrap(),
                val.last().unwrap(),
            )
        })
        .map(|(a, b, c, d)| {
            let mut a = a.to_string();
            a.push_str(".");
            (
                a,
                c.parse::<i32>().unwrap() * if b { 1 } else { -1 },
                d.to_string(),
            )
        })
        .collect::<Vec<(String, i32, String)>>();
    let vals = input
        .iter()
        .map(|(a, b, c)| ((a.as_str(), c.as_str()), *b))
        .collect::<HashMap<(&str, &str), i32>>();
    let mut names = vals
        .iter()
        .map(|((a, b), c)| a)
        .fold(Vec::new(), |mut sum, curr| {
            if !sum.contains(&curr) {
                sum.push(curr);
            }
            sum
        });
    println!("{names:?}");
    let mut max = i32::MIN;
    for arrangement in generateswaps(names) {
      max = cmp::max(evalSeating(&arrangement, &vals), max);
    }
    println!("{max}");
}

fn evalSeating(names: &Vec<&&str>, weights: &HashMap<(&str, &str), i32>) -> i32 {
    names
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| weights.get(&(a, b)).unwrap() + weights.get(&(b, a)).unwrap())
        .sum::<i32>()
}

fn generateswaps<T>(a: Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    if a.len() == 0 {
        return vec![vec![]];
    }
    let mut ret = Vec::new();
    for i in 0..a.len() {
        let mut copy = a.clone();
        let val = copy.remove(i);
        let mut generated = generateswaps(copy);
        for b in 0..generated.len() {
            generated[b].push(val.clone());
            ret.push(generated[b].clone());
        }
    }
    ret
}
