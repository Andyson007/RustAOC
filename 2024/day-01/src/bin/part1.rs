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
    let mut left = lines.iter().map(|x| x.0).collect::<Vec<_>>();
    let mut right = lines.iter().map(|x| x.1).collect::<Vec<_>>();
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .inspect(|x| println!("{x:?}"))
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<u64>()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 11);
    }
}
