use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    let grid = raw
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let antennae = grid
        .iter()
        .flatten()
        .filter(|x| **x != '.')
        .collect::<HashSet<_>>();
    println!("{antennae:?}");
    let mut antinodes = HashSet::new();
    for antenna in antennae {
        for (a, b) in grid
            .iter()
            .enumerate()
            .flat_map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, x)| *x == antenna)
                    .map(move |(j, _)| (i as isize, j as isize))
            })
            .tuple_combinations()
        {
            let diff = (b.0 - a.0, b.1 - a.1);
            antinodes.insert((diff.0 + b.0, diff.1 + b.1));
            antinodes.insert((a.0 - diff.0 , a.1 - diff.1));
        }
    }
    let size = (grid.len() as isize, grid[0].len() as isize);
    println!("{antinodes:?}");
    for i in 0..size.0 {
        for j in 0..size.1 {
            if grid[i as usize][j as usize] == '.' {
                if antinodes.contains(&(i, j)) {
                    print!("#");
                } else {
                    print!(".");
                }
            } else if antinodes.contains(&(i, j)) {
                print!("#");
            } else {
                print!("/");
            }
        }
        println!()
    }
    antinodes
        .into_iter()
        .filter(|(i, j)| *i >= 0 && *j >= 0 && *i < size.0 && *j < size.1)
        .count()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 14)
    }
}
