#[allow(dead_code)]
use core::fmt;
use std::cmp::{max, min};

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

    pub fn is_superset(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        other.is_superset(self)
    }

    pub fn len(&self) -> usize {
        self.end - self.start + 1
    }

    pub fn has_overlap(&self, other: &Self) -> bool {
        self.is_superset(other)
            || self.is_subset(other)
            || (self.start <= other.start && self.end >= other.start)
            || (self.start <= other.end && self.end >= other.end)
    }

    pub fn merge(&self, other: &Self) -> Range {
        debug_assert!(self.has_overlap(other));
        Self {
            start: min(self.start, other.start),
            end: max(self.end, other.end),
        }
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

pub fn solve(input: &str) -> usize {
    let ranges: Vec<_> = input
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(Range::parse)
        .collect();
    let mut merged: Vec<Range> = Vec::new();
    for mut curr in ranges {
        loop {
            if let Some(pos) = merged.iter().position(|x| curr.has_overlap(x)) {
                curr = curr.merge(&merged[pos]);
                merged.swap_remove(pos);
                continue;
            }
            break;
        }
        merged.push(curr);
        // eprintln!("{merged:#?}");
    }
    merged.sort_by_key(|x| x.start);
    eprintln!("{merged:#?}");
    for elem in merged.windows(2) {
        assert!(elem[0].end < elem[1].start, "{elem:?}");
    }

    merged.iter().map(Range::len).sum()
}

#[cfg(test)]
mod test {
    use crate::{solve, Range};

    #[test]
    fn example() {
        let ans = solve(
            r"41738954880549-49586059980007
43245307834664-46019742026916

    ",
        );
        assert_eq!(ans, 14);
    }

    #[test]
    fn overlap() {
        assert!(
            Range {
                start: 43245307834664,
                end:   46019742026916
            }
            .has_overlap(&Range {
                start: 41738954880549,
                end:   49586059980007
            })
        )
    }
}
