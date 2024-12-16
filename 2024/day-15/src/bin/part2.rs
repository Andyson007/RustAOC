use std::{
    collections::HashSet,
    convert::Infallible,
    hash::Hash,
    ops::{Add, AddAssign, Mul, Sub, SubAssign},
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
        .map(|line| {
            line.chars()
                .flat_map(|x| match x {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => unreachable!(),
                })
                .map(Content::new)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut robot_pos = grid
        .iter()
        .enumerate()
        .flat_map(|(i, x)| {
            x.iter()
                .enumerate()
                .map(move |(j, x)| (x, Pos::new(j as isize, i as isize)))
        })
        .find(|x| *x.0 == Content::Robot)
        .unwrap()
        .1;
    for robot_move in moves
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
        if robot_move.y == 0 {
            move_left(&mut grid, &mut robot_pos, robot_move);
        } else {
            move_up(&mut grid, &mut robot_pos, robot_move);
        }
    }
    grid.iter()
        .enumerate()
        .flat_map(|(i, x)| x.iter().enumerate().map(move |(j, x)| (x, (i, j))))
        .filter(|x| *x.0 == Content::BoxLeft)
        .map(|(_, (i, j))| 100 * i + j)
        .sum()
}

fn move_left(grid: &mut [Vec<Content>], robot_pos: &mut Pos, robot_move: Pos) {
    let mut shadow_pos = *robot_pos + robot_move;
    while grid[shadow_pos.y as usize][shadow_pos.x as usize] != Content::Empty {
        if grid[shadow_pos.y as usize][shadow_pos.x as usize] == Content::Wall {
            return;
        }
        shadow_pos += robot_move;
    }
    grid[shadow_pos.y as usize].remove(shadow_pos.x as usize);
    grid[shadow_pos.y as usize].insert(robot_pos.x as usize, Content::Empty);
    *robot_pos += robot_move;
}

fn move_up(grid: &mut [Vec<Content>], robot_pos: &mut Pos, robot_move: Pos) {
    let mut shadow_poses = HashSet::from([*robot_pos + robot_move]);
    let mut to_move = Vec::new();
    while !shadow_poses.is_empty() {
        let mut new_poses = HashSet::new();
        for shadow_pos in shadow_poses {
            match grid[shadow_pos.y as usize][shadow_pos.x as usize] {
                Content::BoxLeft => {
                    to_move.push(shadow_pos + robot_move);
                    to_move.push(shadow_pos + robot_move + Pos::new(1, 0));
                    new_poses.insert(shadow_pos + robot_move);
                    new_poses.insert(shadow_pos + robot_move + Pos::new(1, 0));
                }
                Content::BoxRight => {
                    to_move.push(shadow_pos + robot_move);
                    to_move.push(shadow_pos + robot_move - Pos::new(1, 0));
                    new_poses.insert(shadow_pos + robot_move);
                    new_poses.insert(shadow_pos + robot_move - Pos::new(1, 0));
                }
                Content::Empty => (),
                Content::Wall => return,
                Content::Robot => unreachable!(),
            }
        }
        shadow_poses = new_poses;
    }
    let mut inserted = HashSet::new();
    for pos in to_move.into_iter().rev() {
        if !inserted.insert(pos) {
            continue;
        }
        let pa: *mut Content = &mut grid[pos.y as usize][pos.x as usize];
        let pb: *mut Content =
            &mut grid[(pos - robot_move).y as usize][(pos - robot_move).x as usize];
        unsafe {
            ptr::swap(pa, pb);
        }
    }
    let pa: *mut Content = &mut grid[robot_pos.y as usize][robot_pos.x as usize];
    let pb: *mut Content =
        &mut grid[(*robot_pos + robot_move).y as usize][(*robot_pos + robot_move).x as usize];
    unsafe {
        ptr::swap(pa, pb);
    }
    *robot_pos += robot_move;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Content {
    Robot,
    BoxLeft,
    BoxRight,
    Empty,
    Wall,
}

impl Content {
    pub fn new(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Wall,
            '[' => Self::BoxLeft,
            ']' => Self::BoxRight,
            '@' => Self::Robot,
            _ => unreachable!(),
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::BoxLeft => '[',
            Self::BoxRight => ']',
            Self::Robot => '@',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Sub<Pos> for Pos {
    type Output = Self;

    fn sub(self, rhs: Pos) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
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
    fn big() {
        assert_eq!(solve(include_str!("../../example")), 9021);
    }
}
