use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

fn main() {
    let mut computer = Intcode::from_str(include_str!("../../input"));
    let output = computer.compute(std::iter::once(1));
    println!("{}", output[0]);
}

#[derive(Clone, Debug)]
struct Intcode {
    values: Tape,
    eip: isize,
    relative: isize,
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

    pub fn compute<I>(&mut self, mut inputs: I) -> Vec<isize>
    where
        I: Iterator<Item = isize>,
    {
        let mut ret = Vec::new();
        loop {
            let do_step = self.do_step(&mut inputs);
            match do_step {
                StepResult::Output(x) => {
                    ret.push(x);
                }
                StepResult::Nothing => (),
                StepResult::Finished => break,
            }
        }
        ret
    }

    fn do_step<I>(&mut self, inputs: &mut I) -> StepResult<isize>
    where
        I: Iterator<Item = isize>,
    {
        if self.values[self.eip] == 99 {
            return StepResult::Finished;
        }
        let (opcode, [a, b, c]) = self.parse_modes();
        println!("{opcode}");
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
                    panic!();
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
}

#[derive(Debug, Clone)]
struct Tape {
    values: HashMap<isize, isize>,
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

#[cfg(test)]
mod test {
    use crate::Intcode;

    #[test]
    fn example1() {
        let mut computer = Intcode::new(vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ]);
        let output = computer.compute(std::iter::empty());
        assert_eq!(
            output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        )
    }

    #[test]
    fn example2() {
        let mut computer = Intcode::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        let output = computer.compute(std::iter::empty());
        assert!(output[0].to_string().len() >= 16,)
    }

    #[test]
    fn example3() {
        let mut computer = Intcode::new(vec![104, 1125899906842624, 99]);
        let output = computer.compute(std::iter::empty());
        assert_eq!(output[0], 1125899906842624);
    }
}
