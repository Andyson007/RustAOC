fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| line.split(": ").nth(1).unwrap())
        .map(|line| {
            let values = line
                .split("|")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|string| string.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            line.split("|")
                .nth(0)
                .unwrap()
                .split_whitespace()
                .map(|string| string.trim().parse::<u32>().unwrap())
                .map(|number| values.iter().any(|value| *value == number) as u32)
                .sum::<u32>()
        })
        .filter(|amount| *amount > 0)
        .map(|amount| 1 << (amount - 1))
        .sum::<u32>();
    println!("{ans}");
}
