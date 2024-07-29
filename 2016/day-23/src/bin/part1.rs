fn main() {
    let code = Code::parse(include_str!("../../input"));
    let regs = code.run();
    println!("{regs:?}");
}

#[derive(Debug)]
struct Code {
    code: Vec<Instruction>,
}

impl Code {
    pub fn parse(data: &str) -> Self {
        Self {
            code: data
                .lines()
                .map(|x| {
                    let split = x.split_whitespace().collect::<Vec<&str>>();
                    match split[0] {
                        "cpy" => Instruction::Cpy(Value::from(split[1]), Value::from(split[2])),
                        "inc" => Instruction::Inc(Value::from(split[1])),
                        "dec" => Instruction::Dec(Value::from(split[1])),
                        "jnz" => Instruction::Jnz(Value::from(split[1]), Value::from(split[2])),
                        "tgl" => Instruction::Tgl(Value::from(split[1])),
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<_>>(),
        }
    }
    pub fn run(&self) -> [i32; 4] {
        let mut code = self.code.clone();
        let mut line = 0isize;
        let mut registers = [7, 0, 0, 0];
        while let Some(x) = code.get(line as usize) {
            println!("{registers:?}");
            // for (i, line_code) in code.iter().enumerate() {
            //     if i == line as usize {
            //         print!("\t");
            //     }
            //     println!("{line_code:?}");
            // }
            match x {
                Instruction::Cpy(a, b) => {
                    if let Value::Register(b) = b {
                        registers[(*b as u8 - b'a') as usize] = match a {
                            Value::Register(r) => registers[(*r as u8 - b'a') as usize],
                            Value::Number(x) => *x,
                        }
                    }
                }
                Instruction::Inc(r) => {
                    if let Value::Register(r) = r {
                        registers[(*r as u8 - b'a') as usize] += 1
                    }
                }
                Instruction::Dec(r) => {
                    if let Value::Register(r) = r {
                        registers[(*r as u8 - b'a') as usize] -= 1
                    }
                }
                Instruction::Jnz(a, b) => {
                    if match a {
                        Value::Register(r) => registers[(*r as u8 - b'a') as usize],
                        Value::Number(x) => *x,
                    } != 0
                    {
                        line += match b {
                            Value::Register(r) => registers[(*r as u8 - b'a') as usize],
                            Value::Number(x) => *x,
                        } as isize;
                        continue;
                    }
                }
                Instruction::Tgl(a) => {
                    let index = (line
                        + match a {
                            Value::Register(r) => registers[(*r as u8 - b'a') as usize],
                            Value::Number(x) => *x,
                        } as isize) as usize;
                    if let Some(ref mut x) = code.get_mut(index) {
                        **x = x.toggle();
                    }
                }
            }
            line += 1;
        }
        registers
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Tgl(Value),
}

impl Instruction {
    pub fn toggle(&self) -> Self {
        match self {
            Self::Cpy(a, b) => Self::Jnz(*a, *b),
            Self::Inc(a) => Self::Dec(*a),
            Self::Dec(a) => Self::Inc(*a),
            Self::Jnz(a, b) => Self::Cpy(*a, *b),
            Self::Tgl(a) => Self::Inc(*a),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Register(char),
    Number(i32),
}

impl Value {
    pub fn from(data: &str) -> Self {
        if data.len() == 1 && ('a'..='d').contains(&data.chars().next().unwrap()) {
            Self::Register(data.chars().next().unwrap())
        } else {
            Self::Number(data.parse().unwrap())
        }
    }
}
