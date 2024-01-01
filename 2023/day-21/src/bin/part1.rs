use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
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
    let mut reachable: HashSet<(usize, usize)> = HashSet::from([start]);
    for _ in 0..64 {
        let mut newreachable: HashSet<(usize, usize)> = HashSet::new();
        for pos in reachable.clone() {
            for dir in vec![(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let newpos = (pos.0 as i16 + dir.0, pos.1 as i16 + dir.1);
                if newpos.0 == -1
                    || newpos.1 == -1
                    || newpos.0 == parsed.len() as i16
                    || newpos.1 == parsed[0].len() as i16
                {
                    continue;
                }
                let newpos = (newpos.0 as usize, newpos.1 as usize);
                if parsed[newpos.0][newpos.1] {
                  newreachable.insert(newpos);
                }
            }
        }
        // println!("{newreachable:?}\n");
        reachable = newreachable;
    }
    println!("{}", reachable.len());
}
