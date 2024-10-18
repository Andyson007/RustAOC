use std::ops::{Index, IndexMut};

use day_07::IteratorExt;

fn main() {
    let input = include_str!("../../input")
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    let ans = (0..=4)
        .permutations()
        .map(|x| {
            let mut value = 0;
            for i in x {
                let mut inputs = [i, value].into_iter();
                let mut intcode = Intcode::new(input.clone());
                value = intcode.compute(&mut inputs);
            }
            value
        })
        .max()
        .unwrap();
    println!("{ans}");
}

#[derive(Clone, Debug)]
struct Intcode {
    values: Tape,
    eip: usize,
}

impl Intcode {
    pub fn new(values: Vec<isize>) -> Self {
        Self {
            values: Tape::new(values),
            eip: 0,
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

    pub fn compute<I>(&mut self, mut inputs: I) -> isize
    where
        I: Iterator<Item = isize>,
    {
        loop {
            if let StepResult::Output(x) = self.do_step(&mut inputs) {
                break x;
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
                    self.eip = b.unwrap().value(&self.values) as usize;
                } else {
                    self.eip += 3;
                }
            }
            6 => {
                if a.unwrap().value(&self.values) == 0 {
                    self.eip = b.unwrap().value(&self.values) as usize;
                } else {
                    self.eip += 3;
                }
            }
            3 => {
                *a.unwrap().get_position(&mut self.values).unwrap() = inputs.next().unwrap();
                self.eip += 2;
            }
            4 => {
                let ret = a.unwrap().value(&self.values);
                self.eip += 2;
                return StepResult::Output(ret);
            }
            x => unreachable!("{}", x),
        };
        StepResult::Nothing
    }

    fn parse_modes(&mut self) -> (isize, [Option<OpCode>; 3]) {
        let code = self.values[self.eip];
        let opcode = code % 100;

        let a = self.values.values.get(self.eip + 1);
        let b = self.values.values.get(self.eip + 2);
        let c = self.values.values.get(self.eip + 3);

        (
            opcode,
            [
                a.map(|x| {
                    if (code / 100) % 10 == 0 {
                        OpCode::Position(*x as usize)
                    } else {
                        OpCode::Immediate(*x)
                    }
                }),
                b.map(|x| {
                    if (code / 1000) % 10 == 0 {
                        OpCode::Position(*x as usize)
                    } else {
                        OpCode::Immediate(*x)
                    }
                }),
                c.map(|x| {
                    if (code / 10000) % 10 == 0 {
                        OpCode::Position(*x as usize)
                    } else {
                        OpCode::Immediate(*x)
                    }
                }),
            ],
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum OpCode {
    Immediate(isize),
    Position(usize),
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

enum StepResult<T> {
    Nothing,
    Output(T),
    Finished,
}

#[derive(Debug, Clone)]
struct Tape {
    values: Vec<isize>,
}

impl Tape {
    pub fn new(values: Vec<isize>) -> Self {
        Self { values }
    }
}

impl Index<usize> for Tape {
    type Output = isize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for Tape {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

#[cfg(test)]
mod test {
    use day_07::IteratorExt;

    use crate::Intcode;

    #[test]
    fn example1() {
        let ans = (0..=4)
            .permutations()
            .map(|x| {
                let mut value = 0;
                for i in x {
                    let mut inputs = [i, value].into_iter();
                    let mut intcode = Intcode::new(vec![
                        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
                    ]);
                    println!("test");
                    value = intcode.compute(&mut inputs);
                }
                value
            })
            .max()
            .unwrap();
        assert_eq!(ans, 43210);
    }
}
