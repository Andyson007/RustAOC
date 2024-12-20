use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

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
        .enumerate()
        .map(|x| x.1)
        .map(|line| line.chars().collect::<Vec<_>>())
        .filter(|towel| is_possible(towel, &available))
        .count()
}

fn is_possible(towel: &[char], available: &[Vec<char>]) -> bool {
    let mut to_test = BinaryHeap::from([Reverse(Towel { to_color: towel })]);
    let mut visited = HashSet::new();
    while let Some(towel) = to_test.pop() {
        if visited.contains(&towel.0) {
            continue
        }
        if towel.0.to_color.is_empty() {
            return true;
        }
        for ending in available {
            if towel.0.to_color.ends_with(ending) {
                to_test.push(Reverse(Towel {
                    to_color: &towel.0.to_color[..towel.0.to_color.len() - ending.len()],
                }))
            };
        }
        if !visited.insert(towel.0) {
            continue
        }
    }
    false
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
        assert_eq!(solve(include_str!("../../example")), 6)
    }
}
