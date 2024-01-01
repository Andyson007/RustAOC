use std::collections::{HashMap, HashSet};

fn main() {
    let dir_map: HashMap<(i16, i16), Option<bool>> = HashMap::from([
        ((0, 1), Some(true)),
        ((0, -1), Some(true)),
        ((1, 0), Some(false)),
        ((-1, 0), Some(false)),
    ]);
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let mut visited: HashSet<((usize, usize), Option<bool>)> = HashSet::new();
    visited.insert(((0, 0), None));
    let mut tosearch: Vec<((usize, usize), u32, Option<bool>)> = Vec::new();
    tosearch.push(((0, 0), 0, None));
    loop {
        // tosearch.sort_by(|(_, a, _), (_, b, _)| b.cmp(a));
        // for a in tosearch.clone() {
        //     println!("{a:?}");
        // }
        // println!();
        let curr = tosearch[tosearch.len() - 1];
        visited.insert((curr.0, curr.2));
        if curr.0 == (input.len() - 1, input[0].len() - 1) {
            print!("{curr:?}");
            break;
        }
        tosearch.pop();
        for direction in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if *dir_map.get(&direction).unwrap() == curr.2 {
                continue;
            }
            let mut dist = curr.1;
            for jump in 1..4 {
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
                dist += input[position.0][position.1] as u32;
            }
            for jump in 4..=10 {
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
                dist += input[position.0][position.1] as u32;
                if visited.contains(&(position, *dir_map.get(&direction).unwrap())) {
                    continue;
                }
                let direction = *dir_map.get(&direction).unwrap();
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
