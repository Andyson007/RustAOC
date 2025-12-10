fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}")
}

pub fn solve(input: &str) -> u64 {
    input.lines().map(line).inspect(|x| println!("{x}")).sum()
}

fn line(line: &str) -> u64 {
    calc_len(line, 12)
}

fn calc_len(line: &str, len: usize) -> u64 {
    if len == 0 {
        return 0;
    }

    let (pos, max) = line[..line.len() - len + 1]
        .char_indices()
        .rev()
        .map(|(x, d)| (x, d.to_digit(10).unwrap() as u64))
        .max_by_key(|x| x.1)
        .unwrap();

        10u64.pow(len as u32 - 1) * max + dbg!(calc_len(&line[pos + 1..], len - 1))
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let ans = solve(
            r"987654321111111
811111111111119
234234234234278
818181911112111",
        );
        assert_eq!(ans, 3121910778619);
    }
}
