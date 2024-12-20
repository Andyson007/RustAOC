use std::collections::HashMap;

#[derive(Debug)]
enum Value {
    Register(char),
    Value(i64),
}

#[derive(Debug)]
enum Instruction {
    Two(Mode, char, Value),
    Receive(char),
    Send(Value),
}

#[derive(Debug)]
enum Mode {
    Add,
    Mul,
    Set,
    Mod,
    Jgz,
}

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    let input = parse(raw);
    let mut index0 = 0;
    let mut registers0: HashMap<char, i64> = HashMap::new();
    let mut send0 = Vec::new();
    registers0.insert('p', 0);

    let mut index1 = 0;
    let mut registers1: HashMap<char, i64> = HashMap::new();
    registers1.insert('p', 1);
    let mut send1 = Vec::new();

    let mut count = 0;
    let mut found0;
    let mut found1;
    loop {
        count -= send0.iter().len();
        found0 = program(&input, &mut registers0, &mut index0, &mut send1, &mut send0);
        count += send0.iter().len();
        found1 = program(&input, &mut registers1, &mut index1, &mut send0, &mut send1);
        if !found0 && !found1 {
            break;
        }
    }
    count
}

fn program(
    input: &[Instruction],
    registers: &mut HashMap<char, i64>,
    index: &mut i64,
    rcv: &mut Vec<i64>,
    send: &mut Vec<i64>,
) -> bool {
    let mut count = 0;
    loop {
        match &input[*index as usize] {
            Instruction::Two(mode, a, b) => {
                let right = match b {
                    Value::Value(x) => *x,
                    Value::Register(x) => *registers.get(x).unwrap_or(&0),
                };
                let set = match registers.get_mut(a) {
                    Some(x) => x,
                    None => {
                        registers.insert(*a, 0);
                        registers.get_mut(a).unwrap()
                    }
                };
                match mode {
                    Mode::Add => {
                        *set += right;
                    }
                    Mode::Mul => {
                        *set *= right;
                    }
                    Mode::Set => {
                        *set = right;
                    }
                    Mode::Mod => {
                        *set %= right;
                    }
                    Mode::Jgz => {
                        if *set > 0 {
                            *index += right;
                            count += 1;
                            continue;
                        }
                    }
                }
            }
            Instruction::Receive(x) => {
                if rcv.is_empty() {
                    return count > 0;
                } else {
                    let set = match registers.get_mut(x) {
                        Some(x) => x,
                        None => {
                            registers.insert(*x, 0);
                            registers.get_mut(x).unwrap()
                        }
                    };
                    *set = rcv.remove(0);
                }
            }
            Instruction::Send(x) => {
                let value = match x {
                    Value::Value(x) => *x,
                    Value::Register(x) => *registers.get(x).unwrap_or(&0),
                };
                send.push(value);
            }
        }
        *index += 1;
        count += 1;
    }
}

fn parse(raw: &str) -> Vec<Instruction> {
    raw.lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            match split[0] {
                "add" => Instruction::Two(
                    Mode::Add,
                    split[1].chars().next().unwrap(),
                    match split[2].parse::<i64>() {
                        Ok(x) => Value::Value(x),
                        Err(_) => Value::Register(split[2].chars().next().unwrap()),
                    },
                ),
                "mul" => Instruction::Two(
                    Mode::Mul,
                    split[1].chars().next().unwrap(),
                    match split[2].parse::<i64>() {
                        Ok(x) => Value::Value(x),
                        Err(_) => Value::Register(split[2].chars().next().unwrap()),
                    },
                ),
                "mod" => Instruction::Two(
                    Mode::Mod,
                    split[1].chars().next().unwrap(),
                    match split[2].parse::<i64>() {
                        Ok(x) => Value::Value(x),
                        Err(_) => Value::Register(split[2].chars().next().unwrap()),
                    },
                ),
                "jgz" => Instruction::Two(
                    Mode::Jgz,
                    split[1].chars().next().unwrap(),
                    match split[2].parse::<i64>() {
                        Ok(x) => Value::Value(x),
                        Err(_) => Value::Register(split[2].chars().next().unwrap()),
                    },
                ),
                "set" => Instruction::Two(
                    Mode::Set,
                    split[1].chars().next().unwrap(),
                    match split[2].parse::<i64>() {
                        Ok(x) => Value::Value(x),
                        Err(_) => Value::Register(split[2].chars().next().unwrap()),
                    },
                ),
                "rcv" => Instruction::Receive(split[1].chars().next().unwrap()),
                "snd" => Instruction::Send(match split[1].parse::<i64>() {
                    Ok(x) => Value::Value(x),
                    Err(_) => Value::Register(split[1].chars().next().unwrap()),
                }),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<Instruction>>()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let ans = solve(include_str!("../../example"));
        assert_eq!(ans, 3);
    }
}
