use std::collections::HashMap;

fn main() {
    let input = 368078;
    let mut map: HashMap<(isize, isize), usize> = HashMap::new();
    map.insert((0, 0), 1);
    let mut next = (1, 0);
    let mut ring = 1;
    'outer: loop {
        for _ in 0..ring * 2 - 1 {
            if consider(next, &mut map) >= input {
                println!("{}", consider(next, &mut map));
                break 'outer;
            }
            next.1 += 1;
        }
        for _ in 0..ring * 2 {
            if consider(next, &mut map) >= input {
                println!("{}", consider(next, &mut map));
                break 'outer;
            }
            next.0 -= 1;
        }
        for _ in 0..ring * 2 {
            if consider(next, &mut map) >= input {
                println!("{}", consider(next, &mut map));
                break 'outer;
            }
            next.1 -= 1;
        }
        for _ in 0..ring * 2 {
            if consider(next, &mut map) >= input {
                println!("{}", consider(next, &mut map));
                break 'outer;
            }
            next.0 += 1;
        }
        if consider(next, &mut map) >= input {
            println!("{}", consider(next, &mut map));
            break 'outer;
        }
        next.0 += 1;
        ring += 1;
    }
}

fn consider(loc: (isize, isize), map: &mut HashMap<(isize, isize), usize>) -> usize {
    let mut val = 0;
    for x in -1..=1 {
        for y in -1..=1 {
            if !(x == 0 && y == 0) {
                val += map.get(&(loc.0 + x, loc.1 + y)).unwrap_or(&0);
            }
        }
    }
    println!("{val}");
    map.insert(loc, val);
    val
}
