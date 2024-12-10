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
            for i in 0..2 << r.len() - 1 {
                let mut j = 0;
                if *l
                    == r.iter()
                        .copied()
                        .reduce(|a, b| {
                            j += 1;
                            if i & 1 << (j - 1) != 0 {
                                a * b
                            } else {
                                a + b
                            }
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

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 3749);
    }
}
