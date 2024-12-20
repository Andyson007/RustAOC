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
    Send(char),
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
    let input = include_str!("../../input")
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            match split[0] {
                "add" => Instruction::Two(
                    Mode::Add,
                    split[1].chars().nth(0).unwrap(),
                    match split[2].parse::<i64>() {
                        Ok(x) => Value::Value(x),
                        Err(_) => Value::Register(split[2].chars().nth(0).unwrap()),
                    },
                ),
                "mul" => Instruction::Two(
                    Mode::Mul,
                    split[1].chars().nth(0).unwrap(),
                    match split[2].parse::<i64>() {
                        Ok(x) => Value::Value(x),
                        Err(_) => Value::Register(split[2].chars().nth(0).unwrap()),
                    },
                ),
                "mod" => Instruction::Two(
                    Mode::Mod,
                    split[1].chars().nth(0).unwrap(),
                    match split[2].parse::<i64>() {
                        Ok(x) => Value::Value(x),
                        Err(_) => Value::Register(split[2].chars().nth(0).unwrap()),
                    },
                ),
                "jgz" => Instruction::Two(
                    Mode::Jgz,
                    split[1].chars().nth(0).unwrap(),
                    match split[2].parse::<i64>() {
                        Ok(x) => Value::Value(x),
                        Err(_) => Value::Register(split[2].chars().nth(0).unwrap()),
                    },
                ),
                "set" => Instruction::Two(
                    Mode::Set,
                    split[1].chars().nth(0).unwrap(),
                    match split[2].parse::<i64>() {
                        Ok(x) => Value::Value(x),
                        Err(_) => Value::Register(split[2].chars().nth(0).unwrap()),
                    },
                ),
                "rcv" => Instruction::Receive(split[1].chars().nth(0).unwrap()),
                "snd" => Instruction::Send(split[1].chars().nth(0).unwrap()),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<Instruction>>();
    let mut index = 0;
    let mut registers: HashMap<char, i64> = HashMap::new();
    let mut send = 0;
    loop {
        match &input[index as usize] {
            Instruction::Two(mode, a, b) => {
                let right = match b {
                    Value::Value(x) => *x,
                    Value::Register(x) => *registers.get(&x).unwrap(),
                };
                let set = match registers.get_mut(&a) {
                    Some(x) => x,
                    None => {
                        registers.insert(*a, 0);
                        registers.get_mut(&a).unwrap()
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
                            index += right;
                            continue;
                        }
                    }
                }
            }
            Instruction::Receive(x) => {
                if *registers.get(&x).unwrap() > 0 {
                    println!("{send}");
                    break;
                }
            }
            Instruction::Send(x) => {
                send = *registers.get(&x).unwrap();
            }
        }
        index += 1;
    }
}
