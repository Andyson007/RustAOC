fn main() {
    let input = include_str!("../../input.txt");
    let wins: Vec<u32> = input
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
                .sum()
        })
        .collect();
    let mut amounts: Vec<u32> = Vec::new();
    amounts.resize(input.lines().count(), 1);
    for line in wins.iter().enumerate() {
        for i in 1..=*line.1 {
            amounts[line.0 + i as usize] += amounts[line.0];
        }
    }
    let ans: u32 = amounts.iter().sum();
    println!("{ans}");
}
