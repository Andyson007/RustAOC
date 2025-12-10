//! this doesn't even compute the first line
use core::fmt::Debug;
use std::collections::HashSet;
use std::hash::Hash;

fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut ws = line.split_whitespace();
            // let len = ws.next().unwrap().len() - 2;
            ws.next();
            let buttons: Vec<_> = ws
                .take_while(|x| !x.starts_with('{'))
                .map(Button::parse)
                .collect();
            let grid = Required::parse(line.split_whitespace().last().unwrap());
            solve_line(grid, &buttons)
        })
        .enumerate()
        .map(|(curr, x)| {
            println!("{curr}: {x}");
            x
        })
        .sum()
}

fn solve_line(grid: Required, buttons: &[Button]) -> usize {
    let mut curr = HashSet::from([grid]);
    let mut visited = HashSet::new();
    let mut count = 0;
    loop {
        eprintln!("{count}");
        assert!(!curr.is_empty());
        count += 1;
        let mut next = HashSet::new();
        for elem in curr {
            for button in buttons {
                let mut elem = elem.clone();
                if !elem.sub(button) {
                    continue;
                }
                if elem.is_zero() {
                    return count;
                }
                if visited.insert(elem.clone()) {
                    next.insert(elem);
                }
            }
        }
        curr = next;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Button {
    buttons: u128,
}

impl Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("buttons", &format_args!("{:b}", self.buttons))
            .finish()
    }
}
impl Button {
    pub fn parse(buttons: &str) -> Self {
        let sub = buttons.trim_start_matches('(').trim_end_matches(')');
        Self {
            buttons: sub
                .split(',')
                .map(|x| x.trim().parse::<u128>().unwrap())
                .map(|x| 1 << x)
                .reduce(core::ops::BitXor::bitxor)
                .unwrap(),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Required(Vec<usize>);

impl Required {
    pub fn parse(grid: &str) -> Self {
        let sub = grid.trim_start_matches('{').trim_end_matches('}');
        Self(sub.split(',').map(|x| x.parse().unwrap()).collect())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// returns false if a button was pressed too often
    pub fn sub(&mut self, button: &Button) -> bool {
        let mut button = button.buttons;
        while button != 0 {
            if self.0[button.ilog2() as usize] == 0 {
                return false;
            };
            self.0[button.ilog2() as usize] -= 1;
            button -= 1 << button.ilog2();
        }
        true
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|x| *x == 0)
    }
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let ans = solve(
            r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        assert_eq!(ans, 33);
    }
}
