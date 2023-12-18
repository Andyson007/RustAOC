fn main() {
    let mut input = include_str!("../../input.txt")
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| c == '.')
        .collect::<Vec<bool>>();
    let mut sum = 0;
    for _ in 0..400000 {
        sum += input.iter().filter(|b| **b).count();
        input.insert(0, true);
        input.push(true);
        input = input
            .windows(3)
            .map(|arr| !(arr[0] ^ arr[2]))
            .collect::<Vec<bool>>();
    }
    println!("{sum}");
}
