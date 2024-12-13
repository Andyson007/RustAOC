fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    raw.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .map(|x| solve_single(x, 25))
        .sum()
}

fn solve_single(value: usize, depth: usize) -> usize {
    if depth == 0 {
        1
    } else if value == 0 {
        solve_single(1, depth - 1)
    } else if let Some((x, y)) = split(value) {
        solve_single(x, depth - 1) + solve_single(y, depth - 1)
    } else {
        solve_single(2024 * value, depth - 1)
    }
}

fn split(val: usize) -> Option<(usize, usize)> {
    let len = len(val);
    if len % 2 == 0 {
        let factor = pow(10, len / 2);
        Some((val / factor, val % factor))
    } else {
        None
    }
}

fn pow(base: usize, exp: usize) -> usize {
    let mut ans = 1;
    for _ in 0..exp {
        ans *= base;
    }
    ans
}

fn len(mut val: usize) -> usize {
    for i in 0.. {
        if val == 0 {
            return i;
        }
        val /= 10;
    }
    unreachable!()
}

// #[cfg(test)]
// mod test {
//     use crate::{len, solve};
//
//     #[test]
//     fn example() {
//         let input = include_str!("../../example");
//         let ans = solve(input);
//         assert_eq!(ans, 55312);
//     }
//
//     #[test]
//     fn len_test() {
//         assert_eq!(len(573), 3);
//     }
// }
