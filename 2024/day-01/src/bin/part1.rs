fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}")
}

fn solve(input: &str) -> usize {
    let (mut left, mut right): (Vec<isize>, Vec<isize>) = input
        .lines()
        .map(|x| x.split_whitespace().map(|x| x.parse::<isize>().unwrap()))
        .map(|mut x| (x.next().unwrap(), x.next().unwrap()))
        .unzip();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<usize>()
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
