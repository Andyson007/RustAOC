fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let line = line
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap());
            line.clone().max().unwrap() - line.clone().min().unwrap()
        })
        .sum::<u32>();
    println!("{ans}");
}
