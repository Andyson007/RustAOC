use itertools::Itertools;
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
            if dirs.0 == (0, -1) || dirs.1 == (0, -1) {
                poses.push((start.0, start.1 + 1));
            }
        }
    }
    if start.0 != directions[0].len() - 1 {
        if let Some(dirs) = directions[start.1][start.0 + 1] {
            if dirs.0 == (-1, 0) || dirs.1 == (-1, 0) {
                poses.push((start.0 + 1, start.1));
            }
        }
    }
    let mut found = directions
        .iter()
        .map(|line| line.iter().map(|_| false).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    found[start.0][start.1] = true;
    let mut pos = poses[0];
    found[pos.0][pos.1] = true;
    let mut positions: Vec<(usize, usize)> = vec![((start.0, start.1))];
    loop {
        positions.push(pos);
        let mut newpos = None;
        let dirs = directions[pos.1][pos.0];
        if let Some(dirs) = dirs {
            let (first, second) = (dirs.0, dirs.1);
            let other = (
                (second.0 + pos.0 as i16) as usize,
                (second.1 + pos.1 as i16) as usize,
            );
            if !found[other.0][other.1] {
                if let Some((otherfirst, othersecond)) = directions[other.1][other.0] {
                    if (other.0 + otherfirst.0 as usize == pos.0)
                        && (other.1 + otherfirst.1 as usize == pos.1)
                    {
                        newpos = Some(other);
                        found[other.0][other.1] = true;
                    } else if ((other.0 as i16 + othersecond.0) as usize == pos.0)
                        && ((other.1 as i16 + othersecond.1) as usize == pos.1)
                    {
                        newpos = Some(other);
                        found[other.0][other.1] = true;
                    }
                }
            }
            let other = (
                (first.0 + pos.0 as i16) as usize,
                (first.1 + pos.1 as i16) as usize,
            );
            if !found[other.0][other.1] {
                if let Some((otherfirst, othersecond)) = directions[other.1][other.0] {
                    if ((other.0 as i16 + otherfirst.0) as usize == pos.0)
                        && ((other.1 as i16 + otherfirst.1) as usize == pos.1)
                    {
                        newpos = Some(other);
                        found[other.0][other.1] = true;
                    } else if ((other.0 as i16 + othersecond.0) as usize == pos.0)
                        && ((other.1 as i16 + othersecond.1) as usize == pos.1)
                    {
                        newpos = Some(other);
                        found[other.0][other.1] = true;
                    }
                }
            }
        }
        if newpos.is_none() {
            break;
        }
        pos = newpos.unwrap();
    }
    println!("{:?}", positions[0]);
    println!("{:?}", positions[positions.len() - 1]);
    let mut inside: Vec<Vec<(bool, bool)>> = found
        .iter()
        .map(|line| {
            line.iter()
                .map(|b| (!b, false))
                .collect::<Vec<(bool, bool)>>()
        })
        .collect::<Vec<Vec<(bool, bool)>>>();
    positions
        .iter()
        .circular_tuple_windows()
        .for_each(|(a, b, c)| {
            let (a, b, c) = (
                (a.0 as i16, a.1 as i16),
                (b.0 as i16, b.1 as i16),
                (c.0 as i16, c.1 as i16),
            );
            // println!("{a:?} {b:?} {c:?}");
            let (d, e) = ((b.0 - a.0, b.1 - a.1), (c.0 - b.0, c.1 - b.1));
            // println!("{d:?} {e:?}");
            if d == (-1, 0) {
                if e == (-1, 0) {
                    inside[b.0 as usize][(b.1 - 1) as usize].1 = true;
                } else if e == (0, -1) {
                    //nop
                } else if e == (1, 0) {
                    unreachable!();
                } else if e == (0, 1) {
                    inside[b.0 as usize][(b.1 - 1) as usize].1 = true;
                    inside[(b.0 - 1) as usize][b.1 as usize].1 = true;
                }
            } else if d == (1, 0) {
                if e == (1, 0) {
                    inside[b.0 as usize][(b.1 + 1) as usize].1 = true;
                } else if e == (0, 1) {
                    //nop
                } else if e == (-1, 0) {
                    unreachable!();
                } else if e == (0, -1) {
                    inside[b.0 as usize][(b.1 + 1) as usize].1 = true;
                    inside[(b.0 + 1) as usize][b.1 as usize].1 = true;
                }
            } else if d == (0, -1) {
                if e == (0, -1) {
                    inside[(b.0 + 1) as usize][b.1 as usize].1 = true;
                } else if e == (1, 0) {
                    //nop
                } else if e == (0, 1) {
                    unreachable!();
                } else if e == (-1, 0) {
                    inside[(b.0 + 1) as usize][b.1 as usize].1 = true;
                    inside[b.0 as usize][(b.1 - 1) as usize].1 = true;
                }
            } else if d == (0, 1) {
                if e == (0, 1) {
                    inside[(b.0 - 1) as usize][b.1 as usize].1 = true;
                } else if e == (-1, 0) {
                    //nop
                } else if e == (0, -1) {
                    unreachable!();
                } else if e == (1, 0) {
                    inside[(b.0 - 1) as usize][b.1 as usize].1 = true;
                    inside[b.0 as usize][(b.1 + 1) as usize].1 = true;
                }
            } else {
                unreachable!();
            }
        });
    inside = inside
        .iter()
        .map(|line| {
            line.iter()
                .map(|(a, b)| (*a, *a && *b))
                .collect::<Vec<(bool, bool)>>()
        })
        .collect::<Vec<Vec<(bool, bool)>>>();
    loop {
        let mut newinside = inside.clone();
        for i in 0..inside.len() {
            for j in 0..inside.len() {
                if inside[i][j].1 {
                    newinside[i - 1][j].1 = inside[i - 1][j].0 && true;
                    newinside[i + 1][j].1 = inside[i + 1][j].0 && true;
                    newinside[i][j - 1].1 = inside[i][j - 1].0 && true;
                    newinside[i][j + 1].1 = inside[i][j + 1].0 && true;
                }
            }
        }
        if newinside == inside {
          break;
        }
        inside = newinside;
    }
    let mut count = 0;
    for a in inside.clone() {
        for b in a {
          if b.1 {
            count += 1;
          }
            print!("{}", if b.1 { 1 } else { 0 });
        }
        println!()
    }
    print!("{count}");
}
