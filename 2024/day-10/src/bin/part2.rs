use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    let grid = raw
        .lines()
        .map(|line| {
            line.chars()
                .map(|x| x.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // println!("{grid:?}");
    grid.iter()
        .enumerate()
        .flat_map(|(i, x)| x.iter().enumerate().map(move |(j, c)| ((i, j), c)))
        .filter(|(_, c)| **c == 0)
        .map(|x| x.0)
        .map(|x| from_trail_head(x, &grid))
        .sum::<usize>()
}

fn from_trail_head(start: (usize, usize), grid: &[Vec<u8>]) -> usize {
    let mut poses = HashMap::from([(start, 1)]);
    let mut count = 0;
    while !poses.is_empty() {
        let mut new_poses = HashMap::new();
        for (pos, amount) in poses {
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_pos = (pos.0 as isize + dir.0, pos.1 as isize + dir.1);
                if new_pos.0 < 0 || new_pos.1 < 0 {
                    continue;
                }
                let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                if let Some(x) = grid.get(new_pos.0) {
                    if let Some(x) = x.get(new_pos.1) {
                        let curr = grid[pos.0][pos.1];
                        if *x == curr + 1 {
                            if *x == 9 {
                                count += amount;
                            } else {
                                new_poses
                                    .entry(new_pos)
                                    .and_modify(|x| *x += amount)
                                    .or_insert(amount);
                            }
                        }
                    }
                };
            }
        }
        poses = new_poses;
    }
    count
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 81)
    }
}
