use std::collections::HashMap;

fn main() {
    let input = include_str!("../../example.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u32)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut dists: HashMap<(i16, i16), Vec<((i16, i16), u32)>> = HashMap::new();
    dists.insert((0, 0), vec![((0, 0), 0)]);
    let mut next: Vec<((i16, i16), u32)> = Vec::new();
    next.push(((0, 0), 0));
    'outer: loop {
        next.sort_by(|(_, a), (_, b)| b.cmp(a));
        let curr = *next.last().unwrap();
        if curr.0 == (input.len() as i16 - 1, input[0].len() as i16 - 1) {
            println!("{curr:?}");
            break 'outer;
        }
        println!("{:?}", curr);
        next.pop();
        for direction in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nextpos = (
                curr.0 .0 as i16 + direction.0,
                curr.0 .1 as i16 + direction.1,
            );
            if nextpos.0 < 0
                || nextpos.1 < 0
                || nextpos.0 == input.len() as i16
                || nextpos.1 == input[0].len() as i16
            {
                continue;
            }
            let dist = curr.1 + input[nextpos.0 as usize][nextpos.1 as usize];
            if let Some(x) = dists.get(&nextpos) {
                if let Some(pos) = (*x).iter().position(|x| *x == curr) {
                    println!("{:?}", x[pos]);
                }
            } else {
                let mut pos = curr.0;
                for _ in 0..5 {
                  if pos == (0, 0) {
                    break;
                  }
                  if let Some(x) = dists.get(&pos) {
                      println!("{x:?}");
                  }
                  println!("{pos:?}");
                }
                println!();
                next.push((nextpos, dist));
                dists.insert(nextpos, vec![(curr.0, dist)]);
            }
        }
    }
    println!("finished");
}
