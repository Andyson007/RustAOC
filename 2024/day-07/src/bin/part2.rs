fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    raw.lines()
        .map(|line| {
            let (l, r) = line.split_once(':').unwrap();
            let l = l.parse::<usize>().unwrap();
            let r = r
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (l, r)
        })
        .filter(|(l, r)| {
            for mut i in 0..pow(3, r.len() ) {
                if *l
                    == r.iter()
                        .copied()
                        .reduce(|a, b| {
                            let ret = match i % 3 {
                                0 => a * b,
                                1 => a + b,
                                2 => format!("{a}{b}").parse().unwrap(),
                                _ => unreachable!(),
                            };
                            i /= 3;
                            ret
                        })
                        .unwrap()
                {
                    return true;
                }
            }
            false
        })
        .map(|x| x.0)
        .sum()
}

fn pow(base: usize, exp: usize) -> usize {
    let mut ret = 1;
    for _ in 0..exp {
        ret *= base;
    }
    ret
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 11387);
    }
}
