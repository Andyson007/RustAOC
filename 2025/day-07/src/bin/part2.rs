use std::collections::{HashMap, HashSet};

fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

pub fn solve(input: &str) -> usize {
    let mut lines = input.lines();
    let start = lines
        .next()
        .unwrap()
        .chars()
        .position(|x| x == 'S')
        .unwrap();

    let lines = lines.map(|line| {
        line.chars()
            .enumerate()
            .filter(|x| x.1 == '^')
            .map(|x| x.0)
            .collect::<HashSet<_>>()
    });
    let mut positions = HashMap::from([(start, 1)]);
    for line in lines {
        let mut new_positions = HashMap::new();
        for (pos, count) in positions {
            if line.contains(&pos) {
                new_positions.remove(&pos);
                *new_positions.entry(pos - 1).or_insert(0) += count;
                *new_positions.entry(pos + 1).or_insert(0) += count;
            } else {
                *new_positions.entry(pos).or_insert(0) += count;
            }
        }
        positions = new_positions;
        println!("{positions:?}");
    }
    positions.values().sum()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let ans = solve(
            r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );
        assert_eq!(ans, 40);
    }
}
