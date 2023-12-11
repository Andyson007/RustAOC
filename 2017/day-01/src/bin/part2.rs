fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    let len = input.len();
    let mut sum: u32 = 0;
    for i in 0..(len / 2) {
        if input[i] == input[i + len / 2] {
            sum += input[i];
        }
    }
    println!("{}", sum*2);
}
