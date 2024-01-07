use std::collections::HashMap;

#[derive(Debug)]
enum Line {
    Mask(Vec<Option<bool>>),
    Write((u64, u64)),
}

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let (part1, part2) = line.split_once(" = ").unwrap();
            println!("{part1} {part2}");
            match part1.chars().nth(1).unwrap() {
                'a' => Line::Mask(
                    part2
                        .chars()
                        .map(|x| {
                            if x == 'X' {
                                None
                            } else {
                                Some(x.to_digit(10).unwrap() == 1)
                            }
                        })
                        .collect::<Vec<Option<bool>>>(),
                ),
                'e' => Line::Write((
                    part1
                        .chars()
                        .skip(4)
                        .take_while(|x| x.is_digit(10))
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap(),
                    part2.parse::<u64>().unwrap(),
                )),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<Line>>();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = Vec::new();
    for line in input {
        println!("{line:?}");
        match line {
            Line::Mask(x) => mask = x,
            Line::Write((loc, val)) => {
                let mut topush = 0;
                for (i, current) in mask.iter().rev().enumerate() {
                    match current {
                        None => topush |= val & (1<<i),
                        Some(x) => topush |= (*x as u64)<<i,
                    }
                }
                memory.insert(loc, topush);
            }
        }
    }
    let ans = memory
        .iter()
        .inspect(|x| println!("{x:?}"))
        .map(|(_k, v)| v & 0xfffffffff)
        .sum::<u64>();
    println!("{ans}");
}
