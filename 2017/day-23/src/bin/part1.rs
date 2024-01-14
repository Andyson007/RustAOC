#[derive(Debug)]
enum Value {
    Register(usize),
    Val(isize),
}

#[derive(Debug)]
enum Command {
    Jmp(isize),
    Jnz(usize, Value),
    Mul(usize, Value),
    Set(usize, Value),
    Sub(usize, Value),
}

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            let lft = split[1].chars().nth(0).unwrap();
            match split[0] {
                "set" => Command::Set(
                    lft as usize - b'a' as usize,
                    match split[2].parse::<isize>() {
                        Ok(x) => Value::Val(x),
                        Err(_) => Value::Register(
                            split[2].chars().nth(0).unwrap() as usize - b'a' as usize,
                        ),
                    },
                ),
                "sub" => Command::Sub(
                    lft as usize - b'a' as usize,
                    match split[2].parse::<isize>() {
                        Ok(x) => Value::Val(x),
                        Err(_) => Value::Register(
                            split[2].chars().nth(0).unwrap() as usize - b'a' as usize,
                        ),
                    },
                ),
                "mul" => Command::Mul(
                    lft as usize - b'a' as usize,
                    match split[2].parse::<isize>() {
                        Ok(x) => Value::Val(x),
                        Err(_) => Value::Register(
                            split[2].chars().nth(0).unwrap() as usize - b'a' as usize,
                        ),
                    },
                ),
                "jnz" => {
                    if lft.is_digit(10) {
                        Command::Jmp(split[2].parse::<isize>().unwrap())
                    } else {
                        Command::Jnz(
                            lft as usize - b'a' as usize,
                            match split[2].parse::<isize>() {
                                Ok(x) => Value::Val(x),
                                Err(_) => Value::Register(
                                    split[2].chars().nth(0).unwrap() as usize - b'a' as usize,
                                ),
                            },
                        )
                    }
                }
                _ => unreachable!(),
            }
        })
        .collect::<Vec<Command>>();
    let mut registers = [0; 8];
    let mut index = 0;
    let mut count = 0;
    while index < input.len() {
        match &input[index] {
            Command::Set(a, b) => {
                registers[*a] = match b {
                    Value::Register(x) => registers[*x],
                    Value::Val(x) => *x,
                }
            }
            Command::Jmp(x) => {
                index = (index as isize + x) as usize;
                continue;
            }
            Command::Jnz(a, b) => {
                if registers[*a] != 0 {
                    match b {
                        Value::Val(x) => index = (index as isize + x) as usize,
                        Value::Register(x) => index = (index as isize + registers[*x]) as usize,
                    };
                    continue;
                }
            }
            Command::Mul(a, b) => {
                registers[*a] *= match b {
                    Value::Val(x) => *x,
                    Value::Register(x) => registers[*x],
                };
                count += 1;
            }
            Command::Sub(a, b) => {
                registers[*a] -= match b {
                    Value::Val(x) => *x,
                    Value::Register(x) => registers[*x],
                };
            }
        }
        index += 1;
    }
    println!("{count}");
}
