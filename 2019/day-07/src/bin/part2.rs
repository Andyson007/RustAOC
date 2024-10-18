use std::ops::{Index, IndexMut};

use day_07::IteratorExt;

fn the_thing(input: Vec<isize>) -> isize {
    (5..=9)
        .permutations()
        .map(|x| {
            let mut computers = x
                .into_iter()
                .map(|i| (Intcode::new(input.clone()), Some(i)))
                .collect::<Vec<_>>();
            let mut to_append = Some(0isize);
            let mut last_e_output = None;
            loop {
                for (computer, inputs) in computers.iter_mut() {
                    if let IntCodeOutput::Output(x) =
                        computer.compute(inputs.take().into_iter().chain(to_append))
                    {
                        to_append = Some(x);
                    }
                }
                if let Some(x) = to_append {
                    last_e_output = Some(x);
                }
                if let IntCodeOutput::Finished =
                    computers.last_mut().unwrap().0.compute(std::iter::empty())
                {
                    break last_e_output.unwrap();
                }
            }
        })
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("../../input")
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();
    let ans = the_thing(input);
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

#[derive(Debug)]
enum StepResult<T> {
    Blocking,
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

#[derive(Debug)]
enum IntCodeOutput {
    Blocking,
    Finished,
    Output(isize),
}

#[cfg(test)]
mod test {
    use day_07::IteratorExt;

    use crate::{the_thing, IntCodeOutput, Intcode};

    #[test]
    fn example1() {
        let ans = the_thing(vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ]);
        assert_eq!(ans, 139629729);
    }

    #[test]
    fn examlpe2() {
        let ans = the_thing(vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ]);
        assert_eq!(ans, 18216);
    }
}
