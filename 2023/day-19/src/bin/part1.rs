use std::collections::HashMap;

fn main() {
    let mut input = include_str!("../../input.txt").split("\r\n\r\n");
    let maps = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let name = &line[0..line.find("{").unwrap()];
            let line = &line[line.find("{").unwrap() + 1..line.len() - 1];
            (name, line)
        })
        .collect::<HashMap<&str, &str>>();
    // println!("{maps:#?}");
    let ans = input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            (&line[line.find("{").unwrap() + 1..line.len() - 1])
                .split(",")
                .map(|s| (s.chars().next().unwrap(), s[2..].parse::<u32>().unwrap()))
                .collect::<HashMap<char, u32>>()
        })
        .filter(|line| {
          // println!("{line:?}");
            let mut curr = "in";
            loop {
              // println!("{curr}");
                let conditional = maps.get(&curr).unwrap();
                let mut change = false;
                for c in conditional
                    .split(",")
                    .take(conditional.matches(",").count())
                {
                    let tmp = c.split(":").collect::<Vec<&str>>();
                    let (condition, result) = (tmp[0], tmp[1]);
                    let value = (&condition[2..]).parse::<u32>().unwrap();
                    let c = condition.chars().nth(0).unwrap();
                    // println!("{}", value);
                    // println!("{}", *line.get(&c).unwrap());
                    if match condition.chars().nth(1).unwrap() {
                        '<' => *line.get(&c).unwrap() < value,
                        '>' => *line.get(&c).unwrap() > value,
                        _ => unreachable!(),
                    } {
                        curr = result;
                        change = true;
                        break;
                    }
                }
                if !change {
                    curr = conditional.split(",").last().unwrap();
                }
                if curr == "A" || curr == "R" {
                    return curr == "A";
                }
            }
        })
        .map(|s| {
            let mut sum = 0;
            for (_, a) in s {
                sum += a;
            }
            sum
        })
        .sum::<u32>();
    println!("{ans}");
}
