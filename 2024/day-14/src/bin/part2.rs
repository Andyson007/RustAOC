use std::{
    convert::Infallible,
    io::{self, Read},
    ops::{Add, AddAssign, Mul, SubAssign},
    str::FromStr,
    thread::sleep_ms,
};

fn main() {
    let input = include_str!("../../input");
    solve(input, (101, 103));
}

fn solve(raw: &str, size: (isize, isize)) {
    let mut robots = raw
        .lines()
        .map(Robot::from_str)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    for robot in &robots {
        println!("{robot:?}");
    }
    let mut i = 0;
    loop {
        println!("{i}");
        for j in 0..size.1 {
            for i in 0..size.0 {
                let count = robots.iter().filter(|x| x.pos == Pos::new(i, j)).count();
                if count == 0 {
                    print!(" ");
                } else {
                    print!("\u{2588}");
                }
            }
            println!();
        }
        'outer: loop {
            if i % 1000 == 0 {
                println!("{i}");
            }
            for robot in robots.iter_mut() {
                robot.tick(size);
            }
            i += 1;
            for j in 0..size.1 {
                for i in 0..size.0 {
                    let count = robots.iter().filter(|x| x.pos == Pos::new(i, j)).count();
                    if count > 1 {
                        continue 'outer;
                    }
                }
            }
            break;
        }
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();
    }
}

#[derive(Debug)]
struct Robot {
    pos: Pos,
    velocitiy: Pos,
}

impl Robot {
    pub fn tick(&mut self, size: (isize, isize)) {
        self.pos += self.velocitiy;
        if self.pos.x < 0 {
            self.pos.x += size.0;
        } else if self.pos.x >= size.0 {
            self.pos.x -= size.0;
        }

        if self.pos.y < 0 {
            self.pos.y += size.1;
        } else if self.pos.y >= size.1 {
            self.pos.y -= size.1;
        }
    }
}

impl FromStr for Robot {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(' ').unwrap();
        let pos = Pos::from_str(&p[2..]).unwrap();
        let vel = Pos::from_str(&v[2..]).unwrap();
        Ok(Self {
            pos,
            velocitiy: vel,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl FromStr for Pos {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

impl Mul<isize> for Pos {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Add<Pos> for Pos {
    type Output = Self;

    fn add(self, rhs: Pos) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Pos> for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
