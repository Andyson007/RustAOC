use std::collections::{BinaryHeap, HashSet};

fn main() {
    let password = "lpvhkcbi".as_bytes();
    let mut binary_heap = BinaryHeap::from([Node {
        moves: Vec::new(),
        x: 0,
        y: 0,
        dist: 0,
    }]);
    while let Some(node) = binary_heap.pop() {
        if node.x == 3 && node.y == 3 {
            println!("{node:?}");
            println!("{}", node.moves.iter().map(|x|x.to_char()).collect::<String>());
            break;
        }
        let digest = md5::compute(
            [
                password,
                node.moves
                    .iter()
                    .map(|x| x.to_char())
                    .collect::<String>()
                    .as_bytes(),
            ]
            .concat(),
        );
        let mut moves = digest
            .iter()
            .take(2)
            .flat_map(|x| [x / 16, x % 16])
            .enumerate()
            .map(|(i, x)| if x >= 11 { 1 << i } else { 0 })
            .fold(0, |sum, curr| sum | curr);
        // println!("{digest:?}");
        // println!("{:?}", get_moves(moves));
        if node.y == 0 {
            moves &= !0b0001;
        }
        if node.y == 3 {
            moves &= !0b0010;
        }
        if node.x == 0 {
            moves &= !0b0100;
        }
        if node.x == 3 {
            moves &= !0b1000;
        }
        // println!("{node:?}");
        // println!("{:?}", get_moves(moves));
        // println!();

        for dir in get_moves(moves) {
            binary_heap.push(node.next(dir))
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Node {
    moves: Vec<Move>,
    x: i32,
    y: i32,
    dist: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (4 - other.x + 4 - other.y + other.dist).cmp(&(4 - self.x + 4 - self.y + self.dist))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn next(&self, dir: Move) -> Self {
        Node {
            moves: [self.moves.clone(), vec![dir]].concat(),
            dist: self.dist + 1,
            x: match dir {
                Move::Up | Move::Down => self.x,
                Move::Left => self.x - 1,
                Move::Right => self.x + 1,
            },
            y: match dir {
                Move::Up => self.y - 1,
                Move::Down => self.y + 1,
                Move::Left | Move::Right => self.y,
            },
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
#[repr(u8)]
enum Move {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}

impl From<u8> for Move {
    fn from(value: u8) -> Self {
        match value {
            1 => Move::Up,
            2 => Move::Down,
            4 => Move::Left,
            8 => Move::Right,
            _ => panic!("Only 1 bit of 4 can be set"),
        }
    }
}

impl Move {
    fn to_char(&self) -> char {
        match self {
            Move::Up => 'U',
            Move::Down => 'D',
            Move::Left => 'L',
            Move::Right => 'R',
        }
    }
}

fn get_moves(val: u8) -> HashSet<Move> {
    [1, 2, 4, 8]
        .into_iter()
        .filter(|x| val & *x != 0)
        .map(|x| x.into())
        .collect::<HashSet<Move>>()
}
