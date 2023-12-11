fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| line.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(3)
        .map(|pair| pair[0] + pair[1] + pair[2])
        .collect::<Vec<u32>>()
        .windows(2)
        .map(|pair| (pair[0], pair[1]))
        .filter(|(a, b)| a < b)
        .count();
    println!("{ans}");
}
