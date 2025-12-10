fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

pub fn solve(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let original_count = grid
        .iter()
        .map(|line| line.iter().filter(|x| **x).count())
        .sum::<usize>();

    'outer: loop {
        for line_nr in 0..grid.len() {
            for char_nr in 0..grid[line_nr].len() {
                if !grid[line_nr][char_nr] {
                    continue;
                }
                if can_remove(line_nr, char_nr, &grid) {
                    grid[line_nr][char_nr] = false;
                    continue 'outer;
                }
            }
        }
        break;
    }
    original_count
        - grid
            .into_iter()
            .map(|line| line.into_iter().filter(|x| *x).count())
            .sum::<usize>()
}

fn can_remove(line_nr: usize, char_nr: usize, grid: &[Vec<bool>]) -> bool {
    let mut count = 0;
    for dy in -1isize..=1isize {
        for dx in -1isize..=1isize {
            let Ok(new_x) = usize::try_from(char_nr as isize + dx) else {
                continue;
            };
            let Ok(new_y) = usize::try_from(line_nr as isize + dy) else {
                continue;
            };
            if grid
                .get(new_y)
                .is_some_and(|line| line.get(new_x).is_some_and(|x| *x))
            {
                count += 1
            }
        }
    }
    // count the square we stand on
    count <= 4
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let ans = solve(
            r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.",
        );
        assert_eq!(ans, 43);
    }
}
