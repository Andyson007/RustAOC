use std::cmp;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../input")
        .lines()
        .map(|line| {
            let line = line.split_whitespace().nth(2).unwrap();
            let line = &line[2..(line.len() - 1)];
            let dir = match line.chars().nth(line.len() - 1).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => unreachable!(),
            };
            (
                dir,
                i128::from_str_radix(&line[0..(line.len() - 1)], 16).unwrap(),
            )
        })
        .collect::<Vec<(char, i128)>>();
    let mut pos = (0, 0);
    let mut poses = Vec::new();
    for (dir, dist) in input {
        match dir {
            'L' => pos.1 -= dist,
            'R' => pos.1 += dist,
            'U' => pos.0 -= dist,
            'D' => pos.0 += dist,
            _ => unreachable!(),
        }
        poses.push(pos);
    }
    println!("{poses:?}");
    let sum = (poses
        .into_iter()
        .circular_tuple_windows()
        .map(|((x1, y1), (x2, y2))| determinant([[x1, y1], [x2, y2]])))
    .sum::<i128>()
    .abs()
        / 2;
    println!("{sum}");
}

fn determinant(matrix: [[i128; 2]; 2]) -> i128 {
    matrix[0][0] * matrix[1][1] - matrix[1][0] * matrix[0][1]
}
