use std::{collections::HashSet, ops::Add};

fn main() {
    let input = 1352;
    let f = |pos: Pos| {
        (pos.x * pos.x + 3 * pos.x + 2 * pos.x * pos.y + pos.y + pos.y * pos.y + input).count_ones()
            & 1
            == 0
    };
    let start = Pos { x: 1, y: 1 };
    let mut current = HashSet::from([start]);

    let mut visited = HashSet::from([start]);

    for _ in 0..50{
        let mut new = HashSet::new();
        for x in current {
            for dir in [(0, -1), (0, 1), (1, 0), (-1, 0)] {
                let newpos = x + dir;
                if valid(newpos) && f(newpos) && !visited.contains(&newpos) {
                    visited.insert(newpos);
                    new.insert(newpos);
                };
            }
        }
        current = new;
    }

    println!("{}", visited.len());
    // for x in 0..50 {
    //     for y in 0..50 {
    //         if f(Pos { x, y }) {
    //             print!(".");
    //         } else {
    //             print!("#");
    //         }
    //     }
    //     println!();
    // }
}

fn valid(pos: Pos) -> bool {
    pos.x >= 0 && pos.y >= 0
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Add<(isize, isize)> for Pos {
    type Output = Self;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}
