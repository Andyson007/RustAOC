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
                    'A' => 1,
                    'B' => 2,
                    'C' => 3,
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
                    'X' => 1,
                    'Y' => 2,
                    'Z' => 3,
                    _ => panic!(),
                })
                .nth(0)
                .unwrap();
            if theirs == ours {
                return 3 + ours;
            } else if ours == ((theirs) % 3 + 1) {
                return 6 + ours;
            } else {
                return ours;
            }
        })
        .inspect(|x| println!("{x}"))
        .sum::<u32>();
    println!("{ans}")
}
