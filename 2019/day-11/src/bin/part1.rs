use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

fn main() {
    let mut computer = Intcode::from_str(include_str!("../../input"));
    let mut robot = Robot::default();
    let mut input = None;
    let mut values = HashMap::new();

    let mut robot_mode = Mode::Paint;
    loop {
        match computer.compute(input.take().into_iter()) {
            IntCodeOutput::Blocking => input = Some(*values.get(&(robot.as_tuple())).unwrap_or(&0)),
            IntCodeOutput::Finished => break,
            IntCodeOutput::Output(x) => {
                match robot_mode {
                    Mode::Paint => *values.entry(robot.as_tuple()).or_insert(0) = x,
                    Mode::Move => {
                        match x {
                            0 => robot.dir.turn_left(),
                            1 => robot.dir.turn_right(),
                            _ => unreachable!(),
                        };
                        robot.move_forward();
                    }
                }
                robot_mode.next();
            }
        }
    }
    println!("{}", values.len());
}

enum Mode {
    Paint,
    Move,
}

impl Mode {
    pub fn next(&mut self) {
        match self {
            Mode::Paint => *self = Mode::Move,
            Mode::Move => *self = Mode::Paint,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    x: isize,
    y: isize,
    dir: Direction,
}

impl Default for Robot {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: Direction::Up,
        }
    }
}

impl Robot {
    pub fn move_forward(&mut self) {
        match self.dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }

    pub fn as_tuple(&self) -> (isize, isize) {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn turn_right(&mut self) {
        match self {
            Direction::Up => *self = Self::Right,
            Direction::Down => *self = Self::Left,
            Direction::Right => *self = Self::Down,
            Direction::Left => *self = Self::Up,
        };
    }

    pub fn turn_left(&mut self) {
        match self {
            Direction::Up => *self = Self::Left,
            Direction::Down => *self = Self::Right,
            Direction::Right => *self = Self::Up,
            Direction::Left => *self = Self::Down,
        };
    }
}

#[derive(Clone, Debug)]
struct Intcode {
    values: Tape,
    eip: isize,
    relative: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IntCodeOutput {
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
