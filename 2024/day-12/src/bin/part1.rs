use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    let mut ans = 0;
    let mut visited = HashSet::new();
    let grid = raw
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if visited.contains(&(row, col)) {
                continue;
            }
            visited.insert((row, col));

            let region_type = grid[row][col];
            let mut area = 1;
            let mut perimiter = 0;
            let mut to_visit = HashSet::from([(row, col)]);
            while !to_visit.is_empty() {
                let mut new_visited = HashSet::new();
                for (row, col) in to_visit {
                    for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let new_pos = (row as isize + dir.0, col as isize + dir.1);
                        if new_pos.0 < 0 || new_pos.1 < 0 {
                            perimiter += 1;
                            continue;
                        }
                        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
                        let Some(row_there) = grid.get(new_pos.0) else {
                            perimiter += 1;
                            continue;
                        };
                        let Some(grid_there) = row_there.get(new_pos.1) else {
                            perimiter += 1;
                            continue;
                        };
                        if *grid_there == region_type {
                            if visited.contains(&new_pos) {
                                continue;
                            }
                            area += 1;
                            visited.insert(new_pos);
                            new_visited.insert(new_pos);
                        } else {
                            perimiter += 1;
                        }
                    }
                }
                to_visit = new_visited;
            }
            // println!("{region_type} {area} {perimiter}");
            ans += area * perimiter;
        }
    }
    ans
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn small() {
        let input = "AAAA
BBCD
BBCC
EEEC";
        let ans = solve(input);
        assert_eq!(ans, 140);
    }

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 1930)
    }
}
