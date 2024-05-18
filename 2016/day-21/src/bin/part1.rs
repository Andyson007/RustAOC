use std::fmt::{Display, Formatter};

fn main() {
    let input = include_str!("../../input").lines().map(|line| {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        match (split[0], split[1]) {
            ("swap", "letter") => Oper::SwapChar(
                split[2].chars().nth(0).unwrap(),
                split[5].chars().nth(0).unwrap(),
            ),
            ("swap", "position") => Oper::SwapIndex(
                split[2].parse::<usize>().unwrap(),
                split[5].parse::<usize>().unwrap(),
            ),
            ("move", "position") => Oper::Move(
                split[2].parse::<usize>().unwrap(),
                split[5].parse::<usize>().unwrap(),
            ),
            ("reverse", "positions") => Oper::Reverse(
                split[2].parse::<usize>().unwrap(),
                split[4].parse::<usize>().unwrap(),
            ),
            ("rotate", "based") => Oper::RotateChar(split[6].chars().nth(0).unwrap()),
            ("rotate", "right") => Oper::RotateRight(split[2].parse::<usize>().unwrap()),
            ("rotate", "left") => Oper::RotateLeft(split[2].parse::<usize>().unwrap()),
            x => {
                println!("{x:?}");
                unreachable!();
            }
        }
    });
    let mut arrangement = Arrangement::<8>::new();
    for line in input {
        arrangement.modify(&line);
        println!("{line:?} -> {arrangement}");
    }
    println!("{arrangement}");
}

#[derive(Debug)]
enum Oper {
    Reverse(usize, usize),
    Move(usize, usize),
    SwapIndex(usize, usize),
    SwapChar(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateChar(char),
}

#[derive(Debug)]
struct Arrangement<const N: usize> {
    arrangement: [char; N],
    rotate: usize,
}

impl<const N: usize> Arrangement<N> {
    fn modify(&mut self, oper: &Oper) {
        match *oper {
            Oper::Reverse(a, b) => {
                assert!(a < b);
                let mut x = a;
                let mut y = b;
                while x < y {
                    self.arrangement
                        .swap((x + self.rotate) % N, (y + self.rotate) % N);
                    x += 1;
                    y -= 1;
                }
            }
            Oper::Move(a, b) => {
                if a < b {
                    let mut i = (a + self.rotate) % N;
                    while i != (b + self.rotate) % N {
                        self.arrangement.swap(i, (i + 1) % N);
                        i = (i + 1) % N;
                    }
                } else {
                    let mut i = (a + self.rotate) % N;
                    while i != (b + self.rotate) % N {
                        i = (i + N - 1) % N;
                        self.arrangement.swap(i, (i + 1) % N)
                    }
                }
            }
            Oper::SwapIndex(a, b) => self
                .arrangement
                .swap((a + self.rotate) % N, (b + self.rotate) % N),
            Oper::SwapChar(a, b) => {
                let (a, b) = (
                    self.arrangement.iter().position(|x| *x == a).unwrap(),
                    self.arrangement.iter().position(|x| *x == b).unwrap(),
                );
                self.arrangement.swap(a, b)
            }
            Oper::RotateLeft(x) => self.rotate = (self.rotate + x) % N,

            Oper::RotateRight(x) => self.rotate = (self.rotate + N - x) % N,
            Oper::RotateChar(c) => {
                let amount =
                    (self.arrangement.iter().position(|x| *x == c).unwrap() + N - self.rotate) % N;
                self.modify(&Oper::RotateRight(amount + 1));
                if amount >= 4 {
                    self.modify(&Oper::RotateRight(1));
                }
            }
        }
    }

    fn new() -> Self {
        Self {
            arrangement: ('a'..).take(N).collect::<Vec<char>>().try_into().unwrap(),
            rotate: 0,
        }
    }
}

impl<const N: usize> Display for Arrangement<N> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            (0..N)
                .map(|x| (x + self.rotate) % N)
                .map(|x| self.arrangement[x])
                .collect::<String>()
        )
    }
}
