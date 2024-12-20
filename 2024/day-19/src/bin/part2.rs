use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    let (available, to_create) = raw.split_once("\n\n").unwrap();
    let available = available
        .split(", ")
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    to_create
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .map(|towel| counter(&towel, &available))
        .sum()
}

fn counter(towel: &[char], available: &[Vec<char>]) -> usize {
    let mut to_check = HashMap::new();
    to_check.insert(Towel { to_color: towel }, 1);
    let mut ret = 0;
    while !to_check.is_empty() {
        let mut new_to_check = HashMap::new();
        for (towel, amount) in to_check {
            if towel.to_color.is_empty() {
                ret += amount;
                continue;
            }
            for ending in available {
                if towel.to_color.ends_with(ending) {
                    *new_to_check
                        .entry(Towel {
                            to_color: &towel.to_color[..towel.to_color.len() - ending.len()],
                        })
                        .or_insert(0) += amount;
                };
            }
        }
        to_check = new_to_check;
    }
    ret
}

#[derive(Hash, Debug)]
struct Towel<'a> {
    to_color: &'a [char],
}

impl PartialEq for Towel<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.to_color == other.to_color
    }
}

impl Eq for Towel<'_> {}

impl PartialOrd for Towel<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Towel<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_color.len().cmp(&other.to_color.len())
    }
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        assert_eq!(solve(include_str!("../../example")), 16)
    }
}
