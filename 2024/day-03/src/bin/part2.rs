use regex::Regex;

fn main() {
    let input = include_str!("../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> isize {
    let re = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();
    raw.split("do()")
        .map(|x| x.split("don't()").next().unwrap())
        .map(|x| {
            re.captures_iter(x)
                .map(|x| {
                    let (_, [a, b]) = x.extract();
                    a.parse::<isize>().unwrap() * b.parse::<isize>().unwrap()
                })
                .sum::<isize>()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../example");
        let ans = solve(input);
        assert_eq!(ans, 48);
    }
}
