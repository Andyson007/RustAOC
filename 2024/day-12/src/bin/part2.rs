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

            let mut perimiter = Vec::new();

            let region_type = grid[row][col];
            let mut area = 1;
            let mut to_visit = HashSet::from([(row, col)]);
            while !to_visit.is_empty() {
                let mut new_visited = HashSet::new();
                for (row, col) in to_visit {
                    for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let new_pos_isize = (row as isize + dir.0, col as isize + dir.1);
                        if new_pos_isize.0 < 0 || new_pos_isize.1 < 0 {
                            perimiter.push(((row as isize, col as isize), dir));
                            continue;
                        }
                        let new_pos = (new_pos_isize.0 as usize, new_pos_isize.1 as usize);
                        let Some(row_there) = grid.get(new_pos.0) else {
                            perimiter.push(((row as isize, col as isize), dir));
                            continue;
                        };
                        let Some(grid_there) = row_there.get(new_pos.1) else {
                            perimiter.push(((row as isize, col as isize), dir));
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
                            perimiter.push(((row as isize, col as isize), dir));
                        }
                    }
                }
                to_visit = new_visited;
            }
            let mut edges = 0;
            loop {
                edges += 1;
                let (curr_pos, curr_dir) = perimiter.swap_remove(0);

                let mut new_pos = (curr_pos.0 + curr_dir.1, curr_pos.1 - curr_dir.0);
                while let Some(x) = perimiter.iter().position(|x| *x == (new_pos, curr_dir)) {
                    perimiter.swap_remove(x);
                    new_pos = (new_pos.0 + curr_dir.1, new_pos.1 - curr_dir.0);
                }

                let mut new_pos = (curr_pos.0 - curr_dir.1, curr_pos.1 + curr_dir.0);
                while let Some(x) = perimiter.iter().position(|x| *x == (new_pos, curr_dir)) {
                    perimiter.swap_remove(x);
                    new_pos = (new_pos.0 - curr_dir.1, new_pos.1 + curr_dir.0);
                }
                if perimiter.is_empty() {
                    break;
                }
            }
            ans += area * edges;
        }
    }
    ans
}

fn abs((x, y): (isize, isize)) -> (isize, isize) {
    (x.abs(), y.abs())
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
        assert_eq!(ans, 80);
    }

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 1206)
    }

    #[test]
    fn e() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let ans = solve(input);
        assert_eq!(ans, 236)
    }

    #[test]
    fn xo() {
        test_input(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
            436,
        )
    }

    fn test_input(input: &str, expected: usize) {
        let ans = solve(input);
        assert_eq!(ans, expected);
    }
}
