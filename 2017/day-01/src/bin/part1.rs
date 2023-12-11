fn main() {
    let input = include_str!("../../input.txt").lines().nth(0).unwrap();

    let mut ans = input
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .map(|arr| (arr[0], arr[1]))
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .sum::<u32>();
    if input.chars().nth(0).unwrap() == input.chars().rev().nth(0).unwrap() {
      ans += input.chars().nth(0).unwrap().to_digit(10).unwrap();
    }
    println!("{ans}")
}
