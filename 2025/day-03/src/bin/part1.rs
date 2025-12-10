fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}")
}

pub fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (pos, max) = line[..line.len() - 1]
                .char_indices()
                .rev()
                .map(|(x, d)| (x, d.to_digit(10).unwrap()))
                .max_by_key(|x|x.1)
                .unwrap();
            let other = line[pos + 1..]
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .max()
                .unwrap();
            println!("{pos}:{max}");
            10 * max + other
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let ans = solve(
            r"987654321111111
811111111111119
234234234234278
818181911112111",
        );
        assert_eq!(ans, 357);
    }
}
