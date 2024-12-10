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
    let size = (grid.len() as isize, grid[0].len() as isize);
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
            let mut curr = b;
            while curr.0 >= 0 && curr.1 >= 0 && curr.0 < size.0 && curr.1 < size.1 {
                curr.0 += diff.0;
                curr.1 += diff.1;
            }
            println!("{curr:?}");
            curr.0 -= diff.0;
            curr.1 -= diff.1;
            while curr.0 >= 0 && curr.1 >= 0 && curr.0 < size.0 && curr.1 < size.1 {
                antinodes.insert(curr);
                curr.0 -= diff.0;
                curr.1 -= diff.1;
            }
        }
    }
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
        assert_eq!(ans, 34)
    }
}
