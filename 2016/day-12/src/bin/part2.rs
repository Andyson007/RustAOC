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
                        "cpy" => Instruction::Cpy(
                            Value::from(split[1]),
                            split[2].chars().next().unwrap(),
                        ),
                        "inc" => Instruction::Inc(split[1].chars().next().unwrap()),
                        "dec" => Instruction::Dec(split[1].chars().next().unwrap()),
                        "jnz" => Instruction::Jnz(Value::from(split[1]), Value::from(split[2])),
                        _ => unreachable!(),
                    }
                })
                .collect::<Vec<_>>(),
        }
    }
    pub fn run(&self) -> [i32; 4] {
        let mut line = 0isize;
        let mut registers = [0, 0, 1, 0];
        while let Some(x) = self.code.get(line as usize) {
            match x {
                Instruction::Cpy(a, b) => {
                    registers[(*b as u8 - b'a') as usize] = match a {
                        Value::Register(r) => registers[(*r as u8 - b'a') as usize],
                        Value::Number(x) => *x,
                    }
                }
                Instruction::Inc(r) => registers[(*r as u8 - b'a') as usize] += 1,
                Instruction::Dec(r) => registers[(*r as u8 - b'a') as usize] -= 1,
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
            }
            line += 1;
        }
        registers
    }
}

#[derive(Debug)]
enum Instruction {
    Cpy(Value, char),
    Inc(char),
    Dec(char),
    Jnz(Value, Value),
}

#[derive(Debug)]
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
