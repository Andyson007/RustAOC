fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}")
}

fn solve(raw: &str) -> usize {
    raw.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|mut secret| {
            for _ in 0..2000 {
                secret ^= secret * 64;
                secret %= 16777216;
                secret ^= secret / 32;
                secret %= 16777216;
                secret ^= secret * 2048;
                secret %= 16777216;
            }
            secret
        })
        // .inspect(|x| println!("{x}"))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        assert_eq!(solve("1
10
100
2024"), 37327623);
    }
}
