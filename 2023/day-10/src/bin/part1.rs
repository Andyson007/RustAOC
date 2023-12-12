use std::collections::HashMap;

fn main() {
    let dirs: HashMap<char, Option<((i16, i16), (i16, i16))>> = HashMap::from([
        ('F', Some(((0, 1), (1, 0)))),
        ('7', Some(((0, 1), (-1, 0)))),
        ('J', Some(((0, -1), (-1, 0)))),
        ('L', Some(((0, -1), (1, 0)))),
        ('|', Some(((0, 1), (0, -1)))),
        ('-', Some(((1, 0), (-1, 0)))),
        ('.', None),
        ('S', None),
    ]);
    let mut start: (usize, usize) = (0, 0);
    let directions = include_str!("../../input.txt")
        .lines()
        .enumerate()
        .map(|(j, line)| {
            line.chars()
                .enumerate()
                .inspect(|(i, c)| {
                    if *c == 'S' {
                        start = (*i, j);
                    }
                })
                .map(|(_, c)| *dirs.get(&c).unwrap())
                .collect::<Vec<Option<((i16, i16), (i16, i16))>>>()
        })
        .collect::<Vec<Vec<Option<((i16, i16), (i16, i16))>>>>();
    let mut poses: Vec<(usize, usize)> = Vec::new();
    if start.1 != 0 {
        if let Some(dirs) = directions[start.1 - 1][start.0] {
            if dirs.0 == (0, 1) || dirs.1 == (0, 1) {
            println!("{dirs:?}");
                poses.push((start.0, start.1 - 1));
            }
        }
    }
    if start.0 != 0 {
        if let Some(dirs) = directions[start.1][start.0 - 1] {
            if dirs.0 == (1, 0) || dirs.1 == (1, 0) {
                poses.push((start.0 - 1, start.1));
            }
        }
    }
    if start.1 != directions.len() - 1 {
        if let Some(dirs) = directions[start.1 + 1][start.0] {
            println!("{dirs:?}");
            if dirs.0 == (0, -1) || dirs.1 == (0, -1) {
                poses.push((start.0, start.1 + 1));
            }
        }
    }
    if start.0 != directions[0].len() - 1 {
        if let Some(dirs) = directions[start.1][start.0 + 1] {
            println!("{dirs:?}");
            if dirs.0 == (-1, 0) || dirs.1 == (-1, 0) {
                poses.push((start.0 + 1, start.1));
            }
        }
    }
    let mut dists = directions
        .iter()
        .map(|line| line.iter().map(|_| None).collect::<Vec<Option<u16>>>())
        .collect::<Vec<Vec<Option<u16>>>>();
    dists[start.0][start.1] = Some(0);
    for p in poses.clone() {
        dists[p.0][p.1] = Some(1);
        // println!("{p:?}");
    }
    for a in 0..dists.len() {
        for b in 0..dists[a].len() {
            if let Some(b) = dists[b][a] {
                // print!("{b}");
            } else {
                // print!(".");
            }
        }
        // println!();
    }
    let mut count: u16 = 2;
    loop {
    // for _ in 0..5 {
        let mut newposes: Vec<(usize, usize)> = Vec::new();
        for p in poses.clone() {
             //println!("Position: {p:?}");
            let dirs = directions[p.1][p.0];
            if let Some(dirs) = dirs {
                let (first, second) = (dirs.0, dirs.1);
                let other = (
                    (second.0 + p.0 as i16) as usize,
                    (second.1 + p.1 as i16) as usize,
                );
                 //println!("other: {other:?}");
                if dists[other.0][other.1].is_none() {
                    if let Some((otherfirst, othersecond)) = directions[other.1][other.0] {
                        if (other.0 + otherfirst.0 as usize == p.0)
                            && (other.1 + otherfirst.1 as usize == p.1)
                        {
                             //println!("pushed: {other:?}");
                            newposes.push(other);
                            dists[other.0][other.1] = Some(count);
                        } else if ((other.0 as i16 + othersecond.0) as usize == p.0)
                            && ((other.1 as i16 + othersecond.1) as usize == p.1)
                        {
                             //println!("pushed: {other:?}");
                            newposes.push(other);
                            dists[other.0][other.1] = Some(count);
                        }
                    }
                }
                let other = (
                    (first.0 + p.0 as i16) as usize,
                    (first.1 + p.1 as i16) as usize,
                );
                 //println!("other: {other:?}");
                if dists[other.0][other.1].is_none() {
                    if let Some((otherfirst, othersecond)) = directions[other.1][other.0] {
                         //println!("{other:?}\n{otherfirst:?}");
                        if ((other.0 as i16 + otherfirst.0) as usize == p.0)
                            && ((other.1 as i16 + otherfirst.1) as usize == p.1)
                        {
                             //println!("pushed: {other:?}");
                            newposes.push(other);
                            dists[other.0][other.1] = Some(count);
                        } else if ((other.0 as i16 + othersecond.0) as usize == p.0)
                            && ((other.1 as i16 + othersecond.1) as usize == p.1)
                        {
                             //println!("pushed: {other:?}");
                            newposes.push(other);
                            dists[other.0][other.1] = Some(count);
                        }
                    }
                }
            }
        }
        poses = newposes;
        if poses.len() == 0 {
            break;
        }
        count += 1;
    }
    println!("{}", count - 1);
}
