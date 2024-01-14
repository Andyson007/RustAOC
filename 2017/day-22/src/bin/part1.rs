use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input.txt");
//     let input = "..#
// #..
// ...";
    let mut infected = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(j, x)| {
                    if x == '#' {
                        Some((j as isize, i as isize))
                    } else {
                        None
                    }
                })
                .collect::<HashSet<(isize, isize)>>()
        })
        .fold(HashSet::new(), |mut sum, curr| {
            for val in curr {
                sum.insert(val);
            }
            sum
        });
    let lines = input.lines().count() as isize;
    let mut pos = (lines / 2, lines / 2);

    let dir_map = HashMap::from([(0, (0, -1)), (1, (1, 0)), (2, (0, 1)), (3, (-1, 0))]);
    let mut dir = 0;
    let mut count = 0;
    for _ in 0..10000 {
        if infected.remove(&pos) {
            dir = (dir + 1) % 4;
        } else {
            infected.insert(pos);
            count+=1;
            dir = (dir + 3) % 4;
        }
        pos.0 += dir_map.get(&dir).unwrap().0;
        pos.1 += dir_map.get(&dir).unwrap().1;
    }
    println!("{count}");
}
