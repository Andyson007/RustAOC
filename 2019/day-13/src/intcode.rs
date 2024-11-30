use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

#[derive(Clone, Debug)]
pub struct Intcode {
    pub values: Tape,
    eip: isize,
    relative: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntCodeOutput {
    Blocking,
    Output(isize),
    Finished,
}

impl Intcode {
    pub fn new(values: Vec<isize>) -> Self {
        Self {
            values: Tape::new(values),
            eip: 0,
            relative: 0,
        }
    }

    pub fn from_str(input: &str) -> Self {
        Self::new(
            input
                .lines()
                .next()
                .unwrap()
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect(),
        )
    }

    pub fn compute<I>(&mut self, mut inputs: I) -> IntCodeOutput
    where
        I: Iterator<Item = isize>,
    {
        loop {
            let do_step = self.do_step(&mut inputs);
            match do_step {
                StepResult::Output(x) => {
                    break IntCodeOutput::Output(x);
                }
                StepResult::Blocking => return IntCodeOutput::Blocking,
                StepResult::Nothing => continue,
                StepResult::Finished => break IntCodeOutput::Finished,
            }
        }
    }

    fn do_step<I>(&mut self, inputs: &mut I) -> StepResult<isize>
    where
        I: Iterator<Item = isize>,
    {
        if self.values[self.eip] == 99 {
            return StepResult::Finished;
        }
        let (opcode, [a, b, c]) = self.parse_modes();
        match opcode {
            x @ (1..=2 | 7..=8) => {
                let (Some(a), Some(b), Some(c)) = (a, b, c) else {
                    panic!()
                };
                match x {
                    1 => {
                        *c.get_position(&mut self.values).unwrap() =
                            a.value(&self.values) + b.value(&self.values);
                    }
                    2 => {
                        *c.get_position(&mut self.values).unwrap() =
                            a.value(&self.values) * b.value(&self.values);
                    }
                    7 => {
                        if a.value(&self.values) < b.value(&self.values) {
                            *c.get_position(&mut self.values).unwrap() = 1;
                        } else {
                            *c.get_position(&mut self.values).unwrap() = 0;
                        }
                    }
                    8 => {
                        if a.value(&self.values) == b.value(&self.values) {
                            *c.get_position(&mut self.values).unwrap() = 1;
                        } else {
                            *c.get_position(&mut self.values).unwrap() = 0;
                        }
                    }
                    _ => unreachable!(),
                }
                self.eip += 4;
            }
            5 => {
                if a.unwrap().value(&self.values) != 0 {
                    self.eip = b.unwrap().value(&self.values);
                } else {
                    self.eip += 3;
                }
            }
            6 => {
                if a.unwrap().value(&self.values) == 0 {
                    self.eip = b.unwrap().value(&self.values);
                } else {
                    self.eip += 3;
                }
            }
            3 => {
                let Some(next) = inputs.next() else {
                    return StepResult::Blocking;
                };
                *a.unwrap().get_position(&mut self.values).unwrap() = next;
                self.eip += 2;
            }
            4 => {
                let ret = a.unwrap().value(&self.values);
                self.eip += 2;
                return StepResult::Output(ret);
            }
            9 => {
                self.relative += a.unwrap().value(&self.values);
                self.eip += 2;
            }
            x => unreachable!("{}", x),
        };
        StepResult::Nothing
    }

    fn parse_modes(&mut self) -> (isize, [Option<OpCode>; 3]) {
        let code = self.values[self.eip];
        let opcode = code % 100;

        let values = [1, 2, 3isize];

        (
            opcode,
            values.map(|variable1| {
                self.values
                    .values
                    .get(&(self.eip + variable1))
                    .map(
                        |variable2| match (code / 10isize.pow(variable1 as u32 + 1)) % 10 {
                            0 => OpCode::Position(*variable2),
                            1 => OpCode::Immediate(*variable2),
                            2 => OpCode::Position(*variable2 + self.relative),
                            _ => unreachable!(),
                        },
                    )
            }),
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum OpCode {
    Immediate(isize),
    Position(isize),
}

impl OpCode {
    pub fn value(&self, values: &Tape) -> isize {
        match self {
            OpCode::Immediate(x) => *x,
            OpCode::Position(x) => values[*x],
        }
    }

    pub fn get_position<'a>(&self, values: &'a mut Tape) -> Option<&'a mut isize> {
        match self {
            OpCode::Immediate(_) => None,
            OpCode::Position(x) => Some(&mut values[*x]),
        }
    }
}

#[derive(Debug)]
enum StepResult<T> {
    Nothing,
    Output(T),
    Finished,
    Blocking,
}

#[derive(Debug, Clone)]
pub struct Tape {
    pub values: HashMap<isize, isize>,
}

impl Tape {
    pub fn new(values: Vec<isize>) -> Self {
        Self {
            values: values
                .into_iter()
                .enumerate()
                .map(|(a, b)| (a as isize, b))
                .collect(),
        }
    }
}

impl Index<isize> for Tape {
    type Output = isize;

    fn index(&self, index: isize) -> &Self::Output {
        self.values.get(&index).unwrap_or(&0)
    }
}

impl IndexMut<isize> for Tape {
    fn index_mut(&mut self, index: isize) -> &mut Self::Output {
        if let std::collections::hash_map::Entry::Vacant(e) = self.values.entry(index) {
            e.insert(0);
        }
        self.values.get_mut(&index).unwrap()
    }
}
