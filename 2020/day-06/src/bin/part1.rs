use std::collections::HashSet;

fn main() {
    let ans = include_str!("../../input.txt")
        .split("\r\n\r\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .fold(HashSet::<char>::new(), |mut sum, curr| {
                    for c in curr {
                        sum.insert(c);
                    }
                    sum
                })
                .iter()
                .count() as u32
        })
        .sum::<u32>();
    println!("{ans}");
}
