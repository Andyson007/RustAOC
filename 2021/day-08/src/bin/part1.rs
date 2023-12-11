fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            line.split("|")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|signal| signal.len())
                .fold(0u32, |sum, curr| {
                    for collection in vec![2, 3, 4, 7] {
                        if collection == curr {
                            return sum + 1;
                        }
                    }
                    sum
                })
        })
        .sum::<u32>();
    println!("{ans}");
}
