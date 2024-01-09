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
    let input = include_str!("../../input.txt")
        //     let input = "snd 1
        // snd 2
        // snd p
        // rcv a
        // rcv b
        // rcv c
        // rcv d
        // "
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
    let mut index0 = 0;
    let mut registers0: HashMap<char, i64> = HashMap::new();
    registers0.insert('p', 0);
    let mut index1 = 0;
    let mut registers1: HashMap<char, i64> = HashMap::new();
    registers1.insert('p', 1);
    let mut send0 = Vec::new();
    let mut send1 = Vec::new();
    let mut count = 0;
    loop {
        let found0;
        let found1;
        found0 = program(&input, &mut registers0, &mut index0, &mut send1, &mut send0);
        found1 = program(&input, &mut registers1, &mut index1, &mut send0, &mut send1);
        count += send1.len();
        println!("{count}");
        if !found0 && !found1 {
            break;
        }
    }
    println!("{count}");
}

fn program(
    input: &Vec<Instruction>,
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
                    Value::Register(x) => *registers.get(&x).unwrap_or(&0),
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
                            *index += right;
                            count += 1;
                            continue;
                        }
                    }
                }
            }
            Instruction::Receive(x) => {
                if rcv.len() > 0 {
                    if let Some(x) = registers.get_mut(&x) {
                        *x = rcv.remove(0);
                    } else {
                        registers.insert(*x, rcv.remove(0));
                    }
                } else {
                    return count > 0;
                }
            }
            Instruction::Send(x) => {
                send.push(*registers.get(&x).unwrap_or(&0));
            }
        }
        *index += 1;
        count += 1;
    }
}
