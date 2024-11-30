use itertools::Itertools;
fn main() {
    let input: Vec<(char, usize, u32)> = include_str!("../../input")
        .lines()
        .map(|line| {
            let arr = line.split_whitespace().collect::<Vec<&str>>();
            (
                arr[0].chars().nth(0).unwrap(),
                arr[1].parse::<usize>().unwrap(),
                u32::from_str_radix(&arr[2][2..(arr[2].len() - 1)], 16).unwrap(),
            )
        })
        .collect::<Vec<(char, usize, u32)>>();
    let mut dug: Vec<(usize, usize)> = Vec::new();
    let mut pos = (500, 500);
    dug.push(pos);
    for (dir, amount, color) in input {
        for _ in 0..amount {
            match dir {
                'U' => pos.0 -= 1,
                'R' => pos.1 += 1,
                'D' => pos.0 += 1,
                'L' => pos.1 -= 1,
                _ => unreachable!(),
            }
            dug.push(pos);
        }
    }
    let max = (
        dug.iter().max_by(|(a, _), (b, _)| a.cmp(b)).unwrap().0 + 1,
        dug.iter().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap().1 + 1,
    );
    let mut trench: Vec<Vec<bool>> = Vec::new();
    let mut tmp: Vec<bool> = Vec::new();
    tmp.resize(max.1, false);
    trench.resize(max.0, tmp);
    for d in dug.clone() {
        trench[d.0][d.1] = true;
    }
    // for a in trench.clone() {
    //     for b in a {
    //         print!("{}", if b { '#' } else { '.' });
    //     }
    //     println!("");
    // }
    let mut inside: Vec<Vec<(bool, bool)>> = trench
        .iter()
        .map(|line| {
            line.iter()
                .map(|b| (!b, false))
                .collect::<Vec<(bool, bool)>>()
        })
        .collect::<Vec<Vec<(bool, bool)>>>();
    dug.iter().circular_tuple_windows().for_each(|(a, b, c)| {
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
                if b.1 != 0 {
                    inside[b.0 as usize][(b.1 - 1) as usize].1 = true;
                }
            } else if e == (0, -1) {
                //nop
            } else if e == (1, 0) {
                unreachable!();
            } else if e == (0, 1) {
                if b.1 != 0 {
                    inside[b.0 as usize][(b.1 - 1) as usize].1 = true;
                }
                if b.0 != 0 {
                    inside[(b.0 - 1) as usize][b.1 as usize].1 = true;
                }
            }
        } else if d == (1, 0) {
            if e == (1, 0) {
                if b.1 as usize != inside[0].len() - 1 {
                    inside[b.0 as usize][(b.1 + 1) as usize].1 = true;
                }
            } else if e == (0, 1) {
                //nop
            } else if e == (-1, 0) {
                unreachable!();
            } else if e == (0, -1) {
                if b.1 as usize != inside[0].len() - 1 {
                    inside[b.0 as usize][(b.1 + 1) as usize].1 = true;
                }
                if b.0 as usize != inside.len() - 1 {
                    inside[(b.0 + 1) as usize][b.1 as usize].1 = true;
                }
            }
        } else if d == (0, -1) {
            if e == (0, -1) {
                if b.0 as usize != inside.len() - 1 {
                    inside[(b.0 + 1) as usize][b.1 as usize].1 = true;
                }
            } else if e == (1, 0) {
                //nop
            } else if e == (0, 1) {
                unreachable!();
            } else if e == (-1, 0) {
                if b.0 as usize != inside.len() - 1 {
                    inside[(b.0 + 1) as usize][b.1 as usize].1 = true;
                }
                if b.1 as usize != 0 {
                    inside[b.0 as usize][(b.1 - 1) as usize].1 = true;
                }
            }
        } else if d == (0, 1) {
            if e == (0, 1) {
                if b.0 != 0 {
                    inside[(b.0 - 1) as usize][b.1 as usize].1 = true;
                }
            } else if e == (-1, 0) {
                //nop
            } else if e == (0, -1) {
                unreachable!();
            } else if e == (1, 0) {
                if b.0 != 0 {
                    inside[(b.0 - 1) as usize][b.1 as usize].1 = true;
                }
                if b.1 as usize != inside[0].len() - 1 {
                    inside[b.0 as usize][(b.1 + 1) as usize].1 = true;
                }
            }
        } else {
            // unreachable!();
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
            for j in 0..inside[0].len() {
                if inside[i][j].1 {
                    for pos in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                        let newpos = (i as i16 + pos.0, j as i16 + pos.1);
                        if !(newpos.0 < 0
                            || newpos.1 < 0
                            || newpos.0 == inside.len() as i16
                            || newpos.1 == inside[0].len() as i16)
                        {
                            newinside[newpos.0 as usize][newpos.1 as usize].1 =
                                inside[newpos.0 as usize][newpos.1 as usize].0 && true;
                        }
                    }
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
            // print!("{}", if b.1 { 1 } else { 0 });
        }
        // println!()
    }
    print!("{}", inside.len()*inside[0].len()-count);
}
