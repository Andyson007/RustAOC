use std::cmp::Ordering;

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}")
}

fn solve(raw: &str) -> usize {
    raw.lines()
        .filter(|line| {
            let values = line
                .split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            if check(&values) {
                return true;
            }
            for i in 0..values.len() {
                let mut cloned = values.clone();
                cloned.remove(i);
                if check(&cloned) {
                    println!("{line}");
                    println!("{i}");
                    return true;
                }
            }
            false
        })
        .count()
}

fn check(values: &[isize]) -> bool {
    let order = values
        .windows(2)
        .map(|arr| arr[0].cmp(&arr[1]))
        .collect::<Vec<_>>();
    let first = order[0];
    if first == Ordering::Equal {
        return false;
    }

    if !order.into_iter().all(|x| x == first) {
        return false;
    }
    values.windows(2).all(|arr| arr[0].abs_diff(arr[1]) <= 3)
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 4);
    }
}
