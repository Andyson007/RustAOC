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
                i64::from_str_radix(&line[0..(line.len() - 1)], 16).unwrap(),
            )
        })
        .collect::<Vec<(char, i64)>>();
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
    let mut sum = 0;
    // for _ in 0..poses.len() - 1 {
    {
        let (index, current) = poses
            .iter()
            .circular_tuple_windows::<(_, _, _, _)>()
            .enumerate()
            .filter(|(_i, (a, b, c, d))| {
                let dir1 = ((a.0 - b.0).cmp(&0), (a.1 - b.1).cmp(&0));
                let dir2 = ((c.0 - d.0).cmp(&0), (c.1 - d.1).cmp(&0));
                dir1 == (dir2.0.reverse(), dir2.1.reverse())
            })
            .nth(0)
            .unwrap(); /* .collect::<Vec<char>>(); */
        let mut topleft = *current.0;
        let mut bottomright = *current.0;
        for a in [current.0, current.1, current.2, current.3] {
            println!("{a:?}");
            topleft.0 = cmp::min(topleft.0, a.0);
            topleft.1 = cmp::min(topleft.1, a.1);
            bottomright.0 = cmp::max(bottomright.0, a.0);
            bottomright.1 = cmp::max(bottomright.1, a.1);
        }
        let area = (bottomright.0 - topleft.0) * (bottomright.1 - topleft.1);
        let area = area.abs();
        // todo remove the square that is currently considered 
        // from the poses list decreasing it's size.
        sum += area;
    }
    println!("{sum}");
}
