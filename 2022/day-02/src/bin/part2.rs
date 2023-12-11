use core::panic;

fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let theirs = line
                .split_whitespace()
                .nth(0)
                .unwrap()
                .chars()
                .map(|x| match x {
                    'A' => 0,
                    'B' => 1,
                    'C' => 2,
                    _ => panic!(),
                })
                .nth(0)
                .unwrap();
            let ours = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .chars()
                .map(|x| match x {
                    'X' => 0,
                    'Y' => 1,
                    'Z' => 2,
                    _ => panic!(),
                })
                .nth(0)
                .unwrap();
            println!("{ours} {theirs}");
            match ours {
                0 => (theirs + 2) % 3 + 1,
                1 => theirs + 4,
                2 => (theirs + 1) % 3 + 7,
                _ => unreachable!(),
            }
        })
        .inspect(|x| println!("{x}"))
        .sum::<u32>();
    println!("{ans}")
}
