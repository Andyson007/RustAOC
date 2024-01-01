use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../example.txt");
    let mut start = (0, 0);
    let parsed = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start = (i, j);
                    }
                    c != '#'
                })
                .collect::<Vec<bool>>()
        })
        .collect::<Vec<Vec<bool>>>();
    let mut reachable: HashMap<(usize, usize), HashSet<(i16, i16)>> =
        HashMap::from([(start, HashSet::from([(0, 0)]))]);
    for _ in 0..5000 {
        let mut newreachable: HashMap<(usize, usize), HashSet<(i16, i16)>> = HashMap::new();
        for (pos, chunks) in reachable.clone() {
            for dir in vec![(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let newpos = (
                    (pos.0 + (dir.0 as i16 + parsed.len() as i16) as usize) % parsed.len(),
                    (pos.1 + (dir.1 as i16 + parsed[0].len() as i16) as usize) % parsed[0].len(),
                );
                if parsed[newpos.0][newpos.1] {
                    if let Some(x) = newreachable.get_mut(&newpos) {
                        for c in chunks.clone() {
                            let mut chunk = (
                                (pos.0 as i16 + dir.0) / parsed.len() as i16 + c.0,
                                (pos.1 as i16 + dir.1) / parsed[0].len() as i16 + c.1,
                            );
                            if (pos.0 as i16 + dir.0) < 0 {
                                chunk.0 -= 1;
                            }
                            if (pos.1 as i16 + dir.1) < 0 {
                                chunk.1 -= 1;
                            }
                            x.insert(chunk);
                        }
                    } else {
                        newreachable.insert(
                            newpos,
                            chunks.iter().map(|c| {
                                let mut chunk = (
                                    (pos.0 as i16 + dir.0) / parsed.len() as i16 + c.0,
                                    (pos.1 as i16 + dir.1) / parsed[0].len() as i16 + c.1,
                                );
                                if (pos.0 as i16 + dir.0) < 0 {
                                    chunk.0 -= 1;
                                }
                                if (pos.1 as i16 + dir.1) < 0 {
                                    chunk.1 -= 1;
                                }
                                chunk
                            }).collect::<HashSet<(i16, i16)>>()
                        );
                    }
                }
            }
        }
        // for (k, v) in newreachable.clone() {
        //     println!("{k:?}: {v:?}");
        // }
        // println!();
        reachable = newreachable;
    }
    let mut sum = 0;
    for (_, v) in reachable {
        sum += v.len();
    }
    println!("{}", sum);
}
