fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c == 'R' || c == 'B')
                .fold(0, |sum, curr| sum * 2 + if curr { 1 } else { 0 })
        })
        .max()
        .unwrap();
    print!("{ans}");
}
