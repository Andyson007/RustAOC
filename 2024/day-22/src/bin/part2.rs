use std::cmp;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}")
}

fn solve(raw: &str) -> usize {
    let initials = raw
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut max = 0;
    for i in -9..=9 {
        for j in -9..=9 {
            for k in -9..=9 {
                for l in -9..=9 {
                    max = cmp::max(max, evaluate(&initials, [i, j, k, l]));
                }
            }
            println!("j: {j}");
        }
        println!("i: {i}");
    }
    max
}

fn evaluate(stock: &[usize], sequence: [isize; 4]) -> usize {
    stock
        .par_iter()
        .map(|secret| {
            let mut curr_sequence = [None; 4];
            let mut secret = *secret;
            let mut prev_last = secret % 10;
            for _ in 0..2000 {
                secret = gen_next(secret);
                curr_sequence[0] = curr_sequence[1];
                curr_sequence[1] = curr_sequence[2];
                curr_sequence[2] = curr_sequence[3];
                curr_sequence[3] = Some(secret as isize % 10 - prev_last as isize);
                if curr_sequence
                    .iter()
                    .zip(sequence.iter())
                    .all(|(curr, seq)| curr.is_some_and(|x| x == *seq))
                {
                    return secret % 10;
                }
                prev_last = secret % 10;
            }
            0
        })
        .sum()
}

fn gen_next(mut secret: usize) -> usize {
    secret ^= secret * 64;
    secret %= 16777216;
    secret ^= secret / 32;
    secret %= 16777216;
    secret ^= secret * 2048;
    secret % 16777216
}

#[cfg(test)]
mod test {
    use crate::{evaluate, solve};

    #[test]
    fn example2() {
        assert_eq!(
            solve(
                "1
2
3
2024"
            ),
            23
        );
    }

    #[test]
    fn test_evaluate() {
        assert_eq!(evaluate(&[1, 2, 3, 2024], [-2, 1, -1, 3]), 23)
    }
}
