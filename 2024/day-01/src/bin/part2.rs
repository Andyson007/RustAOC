fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}")
}

fn solve(input: &str) -> u64 {
    let lines = input
        .lines()
        .map(|x| {
            let arr = x
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let left = arr[0];
            let right = arr[1];
            (left, right)
        })
        .collect::<Vec<_>>();
    let left = lines.iter().map(|x| x.0).collect::<Vec<_>>();
    let right = lines.iter().map(|x| x.1).collect::<Vec<_>>();
    left.iter()
        .map(|x| *x as u64 * right.iter().filter(|y| **y == *x).count() as u64)
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 31);
    }
}
