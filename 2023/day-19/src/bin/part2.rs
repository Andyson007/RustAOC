use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Range {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32),
    command: String,
}

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
    let mut sum = 0;
    let mut lines: Vec<Range> = Vec::new();
    // lines.push(Range {
    //     x: (787, 788),
    //     m: (1679, 1680),
    //     a: (1222, 1223),
    //     s: (2876, 2877),
    //     command: String::from("in"),
    // });
    lines.push(Range {
        x: (1, 4001),
        m: (1, 4001),
        a: (1, 4001),
        s: (1, 4001),
        command: String::from("in"),
    });
    while let Some(line) = lines.last() {
        if line.command == "A" {
            let var = (line.x.1 - line.x.0) as u64
                * (line.m.1 - line.m.0) as u64
                * (line.a.1 - line.a.0) as u64
                * (line.s.1 - line.s.0) as u64;
            sum += var;
            lines.pop();
            continue;
        } else if line.command == "R" {
            let var = (line.x.1 - line.x.0) as u64
                * (line.m.1 - line.m.0) as u64
                * (line.a.1 - line.a.0) as u64
                * (line.s.1 - line.s.0) as u64;
            lines.pop();
            continue;
        }

        let map = *maps.get(&(line.command.as_str())).unwrap();
        let mut topush: Vec<Range> = Vec::new();
        let mut copy = line.clone();
        let mut is_brake = false;
        'outer: for c in map.split(",").take(map.matches(",").count()) {
            let (command, dest) = c.split_at(c.find(":").unwrap());
            let value = (&command[2..]).parse::<u32>().unwrap();
            let dest = &dest[1..];
            let tomap = command.chars().nth(0).unwrap();
            let comparison = command.chars().nth(1).unwrap();
            match tomap {
                'x' => match comparison {
                    '<' => {
                        if copy.x.1 < value {
                            let mut a = copy.clone();
                            a.x.1 = value;
                            a.command = dest.to_string();
                            topush.push(a.clone());
                            is_brake = true;
                            break 'outer;
                        } else if copy.x.0 < value {
                            let mut a = copy.clone();
                            a.x.1 = value;
                            copy.x.0 = value;
                            a.command = dest.to_string();
                            topush.push(a.clone());
                        }
                    }
                    '>' => {
                        if copy.x.0 > value {
                            let mut a = copy.clone();
                            a.command = dest.to_string();
                            topush.push(a.clone());
                            is_brake = true;
                            break 'outer;
                        } else if copy.x.1 > value {
                            let mut a = copy.clone();
                            a.x.0 = value+1;
                            copy.x.1 = value+1;
                            a.command = dest.to_string();
                            topush.push(a.clone());
                        }
                    }
                    _ => unreachable!(),
                },
                'm' => match comparison {
                    '<' => {
                        if copy.m.1 < value {
                            let mut a = copy.clone();
                            a.m.0 = value;
                            a.command = dest.to_string();
                            topush.push(a.clone());
                            is_brake = true;
                            break 'outer;
                        } else if copy.m.0 < value {
                            let mut a = copy.clone();
                            a.m.1 = value;
                            copy.m.0 = value;
                            a.command = dest.to_string();
                            topush.push(a.clone());
                        }
                    }
                    '>' => {
                        if copy.m.0 > value {
                            let mut a = copy.clone();
                            a.command = dest.to_string();
                            topush.push(a.clone());
                            is_brake = true;
                            break 'outer;
                        } else if copy.m.1 > value {
                            let mut a = copy.clone();
                            a.m.0 = value+1;
                            copy.m.1 = value+1;
                            a.command = dest.to_string();
                            topush.push(a.clone());
                        }
                    }
                    _ => unreachable!(),
                },
                'a' => match comparison {
                    '<' => {
                        if copy.a.1 < value {
                            let mut a = copy.clone();
                            a.a.0 = value;
                            a.command = dest.to_string();
                            topush.push(a.clone());
                            is_brake = true;
                            break 'outer;
                        } else if copy.a.0 < value {
                            let mut a = copy.clone();
                            a.a.1 = value;
                            copy.a.0 = value;
                            a.command = dest.to_string();
                            topush.push(a.clone());
                        }
                    }
                    '>' => {
                        if copy.a.0 > value {
                            let mut a = copy.clone();
                            a.command = dest.to_string();
                            topush.push(a.clone());
                            is_brake = true;
                            break 'outer;
                        } else if copy.a.1 > value {
                            let mut a = copy.clone();
                            a.a.0 = value+1;
                            copy.a.1 = value+1;
                            a.command = dest.to_string();
                            topush.push(a.clone());
                        }
                    }
                    _ => unreachable!(),
                },
                's' => match comparison {
                    '<' => {
                        if copy.s.1 < value {
                            let mut a = copy.clone();
                            a.s.1 = value;
                            a.command = dest.to_string();
                            topush.push(a.clone());
                            is_brake = true;
                            break 'outer;
                        } else if copy.s.0 < value {
                            let mut a = copy.clone();
                            a.s.1 = value;
                            copy.s.0 = value;
                            a.command = dest.to_string();
                            topush.push(a.clone());
                        }
                    }
                    '>' => {
                        if copy.s.0 > value {
                            let mut a = copy.clone();
                            a.command = dest.to_string();
                            topush.push(a.clone());
                            is_brake = true;
                            break 'outer;
                        } else if copy.s.1 > value {
                            let mut a = copy.clone();
                            a.s.0 = value+1;
                            copy.s.1 = value+1;
                            a.command = dest.to_string();
                            topush.push(a.clone());
                        }
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        lines.pop();
        if !is_brake {
            copy.command = map.split(",").last().unwrap().to_string();
            topush.push(copy);
        }
        for t in topush {
            lines.push(t);
        }
    }
    println!("{sum}");
}
