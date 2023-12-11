use std::{cmp, collections::HashMap};

fn main() {
    let input = include_str!("../../input.txt");
    let moves = input.lines().nth(0).unwrap();
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    input.lines().skip(2).for_each(|line| {
        let loc = line.chars().take(3).collect::<String>();
        let l = line.chars().skip(7).take(3).collect::<String>();
        let r = line.chars().skip(12).take(3).collect::<String>();
        if map.contains_key(&loc) {}
        map.insert(loc, (l, r));
    });
    let startinglocations = map
        .clone()
        .iter()
        .map(|(a, _)| a.clone())
        .filter(|a| a.chars().nth(2).unwrap() == 'A')
        .collect::<Vec<String>>();
    let mut factors: HashMap<u32, u32> = HashMap::new();
    for a in startinglocations {
        // println!("{a}");
        let mut curr = a;
        let mut count = 0;
        // let mut visited: Vec<String> = Vec::new();
        'finished: loop {
            for m in moves.chars() {
                // println!("{curr} {m}");
                if curr.chars().nth(2).unwrap() == 'Z' {
                    // println!("{curr}, {count}");
                    for (p, c) in prime(count) {
                        if factors.contains_key(&p) {
                            *factors.get_mut(&p).unwrap() = cmp::min(c, *factors.get(&p).unwrap());
                        } else {
                            factors.insert(p, c);
                        }
                    }
                    break 'finished;
                    // visited.push(curr.clone());
                }
                curr = match m {
                    'L' => map.get(&curr).unwrap().0.clone(),
                    'R' => map.get(&curr).unwrap().1.clone(),
                    _ => unreachable!(),
                };
                count += 1;
            }
        }
    }
    let mut total: u64 = 1;
    for (p, c) in factors {
        for _ in 0..c {
            total *= p as u64;
        }
    }
    println!("{total}");
}
fn prime(val: u32) -> HashMap<u32, u32> {
    let mut val = val;
    let mut ret: HashMap<u32, u32> = HashMap::new();
    let mut i = 2;
    while val != 1 {
        if val % i == 0 {
            val /= i;
            if !ret.contains_key(&i) {
                ret.insert(i, 1);
            } else {
                *ret.get_mut(&i).unwrap() += 1;
            }
            continue;
        }
        i += 1;
    }
    ret
}
