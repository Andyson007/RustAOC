use std::{
    convert::Infallible,
    mem,
    ops::{Add, AddAssign, Mul, SubAssign},
    ptr,
    str::FromStr,
};

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    let (grid, moves) = raw.split_once("\n\n").unwrap();
    let mut grid = grid
        .lines()
        .map(|line| line.chars().map(Content::new).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut robot_pos = grid
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            x.iter()
                .enumerate()
                .map(move |(j, x)| (x, Pos::new(i as isize, j as isize)))
        })
        .find(|x| *x.0 == Content::Robot)
        .unwrap()
        .1;
    'outer: for robot_move in moves
        .lines()
        .flat_map(str::chars)
        .map(|c| match c {
            '^' => Pos::new(0, -1),
            'v' => Pos::new(0, 1),
            '>' => Pos::new(1, 0),
            '<' => Pos::new(-1, 0),
            x => unreachable!("{}", x as u32),
        })
    {
        let mut shadow_pos = robot_pos + robot_move;
        while grid[shadow_pos.y as usize][shadow_pos.x as usize] != Content::Empty {
            if grid[shadow_pos.y as usize][shadow_pos.x as usize] == Content::Wall {
                continue 'outer;
            }
            shadow_pos += robot_move;
        }
        let pa: *mut Content = &mut grid[shadow_pos.y as usize][shadow_pos.x as usize];
        let pb: *mut Content =
            &mut grid[(robot_pos + robot_move).y as usize][(robot_move + robot_pos).x as usize];
        unsafe {
            ptr::swap(pa, pb);
        }
        let pa: *mut Content = &mut grid[robot_pos.y as usize][robot_pos.x as usize];
        let pb: *mut Content =
            &mut grid[(robot_pos + robot_move).y as usize][(robot_move + robot_pos).x as usize];
        unsafe {
            ptr::swap(pa, pb);
        }
        robot_pos += robot_move;
    }
    grid.iter()
        .enumerate()
        .flat_map(|(i, x)| x.iter().enumerate().map(move |(j, x)| (x, (i, j))))
        .filter(|x| *x.0 == Content::Box)
        .map(|(_, (i, j))| 100 * i + j)
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Content {
    Robot,
    Box,
    Empty,
    Wall,
}

impl Content {
    pub fn new(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            'O' => Self::Box,
            '@' => Self::Robot,
            _ => unreachable!(),
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::Box => 'O',
            Self::Robot => '@',
        }
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

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn small() {
        assert_eq!(
            solve(
                "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
            ),
            2028
        );
    }

    #[test]
    fn big() {
        assert_eq!(solve(include_str!("../../example")), 10092);
    }
}
