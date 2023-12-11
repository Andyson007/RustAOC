fn main() {
    let input = include_str!("../../input.txt");
    let time = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .fold(String::from(""), |sum, curr| sum + curr)
        .parse::<u64>()
        .unwrap();
    let max = input
        .lines()
        .nth(1)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .fold(String::from(""), |sum, curr| sum + curr)
        .parse::<u64>()
        .unwrap();
    let mut count = 0;
    for i in 0..=time {
        if i * (time - i) >= max {
            count += 1;
        }
    }
    println!("{count}");
}
