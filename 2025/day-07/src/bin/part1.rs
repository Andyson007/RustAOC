use std::collections::HashSet;

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
    let mut positions = HashSet::from([start]);
    let mut split_count = 0;
    for line in lines {
        let mut new_positions = HashSet::new();
        for pos in positions {
            if line.contains(&pos) {
                new_positions.remove(&pos);
                new_positions.insert(pos - 1);
                new_positions.insert(pos + 1);
                split_count += 1;
            } else {
                new_positions.insert(pos);
            }
        }
        positions = new_positions;
        println!("{positions:?}");
    }
    split_count
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
        assert_eq!(ans, 21);
    }
}
