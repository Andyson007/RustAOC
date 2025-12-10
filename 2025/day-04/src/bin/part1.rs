fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

pub fn solve(input: &str) -> usize {
    let lines = input.lines().collect::<Vec<&str>>();
    lines
        .iter()
        .enumerate()
        .map(|(line_nr, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(char_nr, c)| if c == '@' { Some(char_nr) } else { None })
                .filter(|char_nr| {
                    let mut count = 0;
                    for dy in -1isize..=1isize {
                        for dx in -1isize..=1isize {
                            let Ok(new_x) = usize::try_from(*char_nr as isize + dx) else {
                                continue;
                            };
                            let Ok(new_y) = usize::try_from(line_nr as isize + dy) else {
                                continue;
                            };
                            if lines.get(new_y).is_some_and(|line| {
                                line.chars().nth(new_x).is_some_and(|x| x == '@')
                            }) {
                                count += 1
                            }
                        }
                    }
                    eprintln!("{line_nr},{char_nr}: {count}");
                    // count the square we stand on
                    count <= 4
                })
                .count()
        })
        .sum()
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
        assert_eq!(ans, 13);
    }
}
