use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input, 75);
    println!("{ans}");
}

fn solve(raw: &str, depth: usize) -> usize {
    let mut values = HashMap::new();
    for elem in raw.split_whitespace().map(|x| x.parse::<usize>().unwrap()) {
        if let Some(x) = values.get_mut(&elem) {
            *x += 1;
        } else {
            values.insert(elem, 1usize);
        }
    }
    for _ in 0..depth {
        let mut new_values = HashMap::new();
        for (key, amount) in values {
            for to_insert in {
                if key == 0 {
                    vec![1]
                } else if let Some((x, y)) = split(key) {
                    vec![x, y]
                } else {
                    vec![2024 * key]
                }
            } {
                if let Some(x) = new_values.get_mut(&to_insert) {
                    *x += amount;
                } else {
                    new_values.insert(to_insert, amount);
                }
            }
        }
        values = new_values;
    }
    values.values().sum()
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

#[cfg(test)]
mod test {
    use crate::{len, solve};

    #[test]
    fn example() {
        let input = include_str!("../../example");
        println!("test");
        let ans = solve(input, 25);
        assert_eq!(ans, 55312);
    }

    // #[test]
    // fn part1() {
    //     let input = include_str!("../../input");
    //     let ans = solve(input, 25);
    //     assert_eq!(ans, 193269);
    // }

    #[test]
    fn len_test() {
        assert_eq!(len(573), 3);
    }
}
