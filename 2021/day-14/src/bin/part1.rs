use std::collections::HashMap;

fn main() {
    let mut input = include_str!("../../input").split("\n\n");
    let mut polymer = input.next().unwrap().chars().collect::<Vec<char>>();
    let transformations = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut split = line.split(" -> ");
            let left = split.next().unwrap().chars().take(2).collect::<Vec<char>>();
            (left, split.next().unwrap().chars().next().unwrap())
        })
        .collect::<HashMap<Vec<char>, char>>();
    for _ in 0..10 {
        polymer = polymer
            .windows(2)
            .map(|arr| (transformations.get(arr).unwrap(), &arr[1]))
            .fold(vec![polymer[0]], |mut sum, (a, b)| {
                sum.push(*a);
                sum.push(*b);
                sum
            });
    }
    let counts = polymer.iter().fold([0; 26], |mut sum, curr| {
        sum[(*curr as u8) as usize - 65] += 1;
        sum
    });

    for (i,item) in counts.iter().enumerate() {
        println!("{}: {item}", (i+65) as u8 as char);
    }

    println!(
        "{}",
        dbg!(counts.iter().max().unwrap()) - dbg!(counts.iter().filter(|x| **x != 0).min().unwrap())
    );
}
