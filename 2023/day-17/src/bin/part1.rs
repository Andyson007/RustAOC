use std::collections::{HashMap, HashSet};

fn main() {
    let dir_map: HashMap<(i16, i16), usize> =
        HashMap::from([((0, 1), 0), ((0, -1), 1), ((1, 0), 2), ((-1, 0), 3)]);
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut visited: HashSet<((usize, usize), usize)> = HashSet::new();
    visited.insert(((0, 0), 4));
    let mut tosearch: Vec<((usize, usize), u32, (i16, i16))> = Vec::new();
    tosearch.push(((0, 0), 0, (0, 0)));
    // for _ in 0..3 {
    loop {
        // tosearch.sort_by(|(_, a, _), (_, b, _)| b.cmp(a));
        // for a in tosearch.clone() {
        //     println!("{a:?}");
        // }
        // println!();
        let curr = tosearch[tosearch.len() - 1];
        if curr.0 == (input.len() - 1, input[0].len() - 1) {
            print!("{curr:?}");
            break;
        }
        tosearch.pop();
        for direction in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if direction == curr.2 || (-direction.0, -direction.1) == curr.2 {
                continue;
            }
            let mut dist = curr.1;
            for jump in 1..=3 {
                let position = (
                    curr.0 .0 as i16 + direction.0 * jump,
                    curr.0 .1 as i16 + direction.1 * jump,
                );
                if position.0 < 0
                    || position.1 < 0
                    || position.0 >= input.len() as i16
                    || position.1 >= input[0].len() as i16
                {
                    continue;
                }
                let position = (position.0 as usize, position.1 as usize);
                dist += input[position.0][position.1];
                if visited.contains(&(position, *dir_map.get(&direction).unwrap())) {
                    continue;
                }
                visited.insert((curr.0, *dir_map.get(&direction).unwrap()));
                if let Some(pos) = tosearch
                    .iter()
                    .position(|a| *a == (position, dist, direction))
                {
                    tosearch[pos].1 = dist;
                } else {
                    let index = tosearch.iter().position(|x| x.1 <= dist);
                    match index {
                        None => tosearch.push((position, dist, direction)),
                        Some(x) => tosearch.insert(x, (position, dist, direction)),
                    }
                    // println!("{index:?} {dist}");
                }
            }
        }
    }
}
