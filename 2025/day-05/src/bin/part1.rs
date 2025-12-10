fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn parse(line: &str) -> Self {
        let (start, end) = line.trim().split_once('-').unwrap();
        Self {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }

    pub fn contains(&self, val: usize) -> bool {
        self.start <= val && val <= self.end
    }
}

pub fn solve(input: &str) -> usize {
    let (ranges, items) = input.split_once("\n\n").unwrap();
    let ranges: Vec<_> = ranges.lines().map(Range::parse).collect();
    items
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .filter(|item| ranges.iter().any(|x| x.contains(*item)))
        .count()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let ans = solve(
            r"3-5
10-14
16-20
12-18

1
5
8
11
17
32",
        );
        assert_eq!(ans, 3);
    }
}
