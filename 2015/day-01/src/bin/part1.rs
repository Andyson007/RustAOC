fn main() {
    let ans: i32 = include_str!("../../input")
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| match x {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .sum();
    println!("{ans}")
}
