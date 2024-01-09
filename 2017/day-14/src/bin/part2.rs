use std::collections::HashSet;

fn main() {
    let input = "oundnydw";
    let mut grid = [0u128; 128];
    for i in 0..128 {
        let mut to_eval = String::from(input);
        to_eval.push('-');
        to_eval.push_str(i.to_string().as_str());
        let input = to_eval
            .chars()
            .map(|c| c as u8)
            .chain([17, 31, 73, 47, 23])
            .collect::<Vec<u8>>();
        let mut current: u8 = 0;
        let mut skip: u8 = 0;
        let mut list = (0..=255).collect::<Vec<u8>>();
        for _ in 0..64 {
            for i in &input {
                for j in 0..i / 2 {
                    list.swap((current + j) as usize, (current + i - j - 1) as usize);
                }
                current += i + skip;
                skip += 1;
            }
        }
        grid[i] = list
            .chunks(16)
            .map(|x| x.iter().fold(0, |a, b| a ^ b))
            .fold(0u128, |sum, curr| sum << 8 | curr as u128)
    }
    let mut count = 0;
    while grid.iter().any(|x| *x != 0) {
        count += 1;
        let mut x = 0;
        let mut y = 0;
        for (i, line) in grid.iter().enumerate() {
            if *line != 0 {
                y = i;
                x = line.leading_zeros() as usize;
                break;
            }
        }
        let mut next = HashSet::from([(x, y)]);
        grid[y] ^= 1 << (128 - x - 1) as u128;
        while !next.is_empty() {
            let mut new = HashSet::new();
            for pos in next {
                for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let newpos = (pos.0 as i8 + dir.0, pos.1 as i8 + dir.1);
                    if newpos.0 < 0 || newpos.1 < 0 {
                        continue;
                    }
                    let newpos = (newpos.0 as usize, newpos.1 as usize);
                    if newpos.0 == 128 || newpos.1 == 128 {
                        continue;
                    }
                    if (grid[newpos.1] >> (128 - newpos.0 - 1)) & 1 == 1 {
                        new.insert(newpos);
                        grid[newpos.1] ^= 1 << (128 - newpos.0 - 1) as u128;
                    }
                }
            }
            next = new;
        }
    }
    println!("{count}");
}
