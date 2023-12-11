fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .map(|val|val/3-2)
        .sum::<u32>();
    println!("{ans}")
}
