use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<i8>>()
        })
        .collect::<Vec<Vec<i8>>>();

    let mut visible = HashSet::new();

    for i in 0..input.len() {
        let mut curr = -1i8;
        for j in 0..input[i].len() {
            if input[i][j] > curr {
                visible.insert((i, j));
                curr = input[i][j];
            }
        }
    }

    for i in 0..input.len() {
        let mut curr = -1i8;
        for j in (0..input[i].len()).rev() {
            if input[i][j] > curr {
                visible.insert((i, j));
                curr = input[i][j];
            }
        }
    }

    for i in 0..input[0].len() {
        let mut curr = -1i8;
        for j in 0..input.len() {
            if input[j][i] > curr {
                visible.insert((j, i));
                curr = input[j][i];
            }
        }
    }


    for i in 0..input[0].len() {
        let mut curr = -1i8;
        for j in (0..input.len()).rev() {
            if input[j][i] > curr {
                visible.insert((j, i));
                curr = input[j][i];
            }
        }
    }
    println!("{}", visible.len());
}
