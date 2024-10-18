use std::ops::{Index, IndexMut};

fn main() {
    let input = include_str!("../../input")
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut intcode = Intcode::new(input);
    let ans = intcode.compute(std::iter::once(1));
    println!("{ans}");
}

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

    pub fn compute<I>(&mut self, mut inputs: I) -> isize
    where
        I: Iterator<Item = isize>,
    {
        loop {
            if let StepResult::Output(x) = self.do_step(&mut inputs) {
                if x != 0 {
                    break x;
                }
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
            x @ 1..=2 => {
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
                    _ => unreachable!(),
                }
                self.eip += 4;
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
            _ => unreachable!(),
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

#[derive(Debug)]
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

// #[cfg(test)]
// mod test {
//     use crate::Intcode;
//
//     #[test]
//     fn immediate_exit() {
//         let mut intcode = Intcode::new(vec![99]);
//         intcode.compute(std::iter::empty());
//         assert_eq!(intcode.values.values, vec![99])
//     }
//
//     #[test]
//     fn only_immediate() {
//         let mut intcode = Intcode::new(vec![1101, 5, 4, 0, 99]);
//         intcode.compute(std::iter::empty());
//         assert_eq!(intcode.values.values, vec![9, 5, 4, 0, 99])
//     }
//
//     #[test]
//     fn relative() {
//         let mut intcode = Intcode::new(vec![1, 2, 1, 0, 99]);
//         intcode.compute(std::iter::empty());
//         assert_eq!(intcode.values.values, vec![3, 2, 1, 0, 99])
//     }
//
//     #[test]
//     fn input() {
//         let mut intcode = Intcode::new(vec![3, 1, 99]);
//         intcode.compute(std::iter::once(5));
//         assert_eq!(intcode.values.values, vec![3, 5, 99])
//     }
// }
