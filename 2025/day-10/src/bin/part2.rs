#![feature(test)]
use core::fmt::Debug;
use core::iter;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

pub fn solve(input: &str) -> u16 {
    input
        .lines()
        .map(wrap_line)
        .inspect(|x| println!("{x}"))
        .sum()
}

pub fn wrap_line(line: &str) -> u16 {
    let mut ws = line.split_whitespace();
    ws.next();
    let buttons: Vec<_> = ws
        .take_while(|x| !x.starts_with('{'))
        .map(Button::parse)
        .collect();
    let grid = Required::parse(line.split_whitespace().last().unwrap());
    solve_parsed_line(grid, &buttons)
}

pub fn solve_parsed_line(mut grid: Required, buttons: &[Button]) -> u16 {
    // returns all buttons which modify the current button and only future ones
    let get_modifiers = move |led| {
        buttons
            .iter()
            .filter(move |button| button.0.contains(&led))
            .filter(move |button| button.0.iter().all(|x| *x >= led))
            .collect()
    };
    let mut visited = HashMap::new();
    let mut trivals = 0;
    for i in 0..grid.0.len() {
        if buttons.iter().filter(|x| x.0.contains(&i)).count() == 1 {
            let b = buttons.iter().find(|x| x.0.contains(&i)).unwrap();
            trivals += grid.0[i];
            grid.sub(&b.0, grid.0[i]);
        }
    }
    print!("{trivals}: ");
    solve_step(0, &grid, get_modifiers, &mut visited).unwrap() + trivals
}

fn solve_step<'a>(
    curr: usize,
    grid: &Required,
    get_modifiers: impl Fn(usize) -> Vec<&'a Button> + Copy,
    visited: &mut HashMap<Required, u16>,
) -> Option<u16> {
    if let Some(x) = visited.get(grid) {
        return Some(*x);
    }
    if grid.is_zero() {
        return Some(0);
    }
    if curr == grid.0.len() {
        return None;
    }
    let available = &get_modifiers(curr);
    Combinations::new(available.len(), grid.0[curr] as usize)
        .filter_map(|combination| {
            let mut grid = grid.clone();
            for (b, amount) in available.iter().zip(combination) {
                if !grid.sub(&b.0, amount) {
                    return None;
                }
            }
            solve_step(curr + 1, &grid, get_modifiers, visited)
        })
        .min()
        .map(|x| x + grid.0[curr])
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Button(Vec<usize>);

impl Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}
impl Button {
    pub fn parse(buttons: &str) -> Self {
        let sub = buttons.trim_start_matches('(').trim_end_matches(')');
        Self(sub.split(',').map(|x| x.trim().parse().unwrap()).collect())
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Required(Vec<u16>);

impl Debug for Required {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(&self.0).finish()
    }
}

impl Required {
    pub fn parse(grid: &str) -> Self {
        let sub = grid.trim_start_matches('{').trim_end_matches('}');
        Self(sub.split(',').map(|x| x.parse().unwrap()).collect())
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|x| *x == 0)
    }

    /// Returns false if this would overpress a button
    /// true if everything is fine
    pub fn sub(&mut self, button: &[usize], amount: u16) -> bool {
        for b in button {
            if self.0[*b] < amount {
                return false;
            }
            self.0[*b] -= amount;
        }
        true
    }
}

pub struct Combinations {
    lens: Vec<u16>,
    needed: usize,
    done: bool,
}

impl Combinations {
    pub fn new(items: usize, amount: usize) -> Self {
        if items == 0 {
            Self {
                lens: Vec::new(),
                needed: amount,
                done: false,
            }
        } else {
            Self {
                lens: iter::repeat_n(0, items - 1).collect(),
                needed: amount,
                done: false,
            }
        }
    }
}

impl Iterator for Combinations {
    type Item = Vec<u16>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let clone = {
            let mut clone = self.lens.clone();
            let rest = self.needed as u16 - self.lens.iter().sum::<u16>();
            clone.push(rest);
            clone
        };

        if self.lens.is_empty() {
            self.done = true;
            return Some(clone);
        }
        let ret = Some(clone);
        self.lens[0] += 1;
        let mut pos = 0;
        while self.lens.iter().sum::<u16>() > self.needed as u16 {
            self.lens[pos] = 0;
            let Some(next) = self.lens.get_mut(pos + 1) else {
                self.done = true;
                return ret;
            };
            *next += 1;
            pos += 1;
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use crate::{Button, Required, solve, solve_parsed_line};

    #[test]
    fn example() {
        let ans = solve(
            r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        );
        assert_eq!(ans, 33);
    }

    #[test]
    fn test_first() {
        let req = Required(vec![3, 5, 4, 7]);
        let buttons = [
            Button(vec![3]),
            Button(vec![1, 3]),
            Button(vec![2]),
            Button(vec![2, 3]),
            Button(vec![0, 2]),
            Button(vec![0, 1]),
        ];
        let ans = solve_parsed_line(req, &buttons);
        assert_eq!(ans, 10)
    }

    #[test]
    fn test_second() {
        let req = Required(vec![7, 5, 12, 7, 2]);
        let buttons = [
            Button(vec![0, 2, 3, 4]),
            Button(vec![2, 3]),
            Button(vec![0, 4]),
            Button(vec![0, 1, 2]),
            Button(vec![1, 2, 3, 4]),
        ];
        let ans = solve_parsed_line(req, &buttons);
        assert_eq!(ans, 12)
    }

    #[test]
    fn input_five() {
        solve(
            "[.#.##.#.] (6,7) (0,2) (0,3,4,5,6,7) (0,1,2,3) (2) (1,2,4,5,7) (2,4,5,6,7) (0,1,3,5,7) {47,36,44,43,19,31,162,180}",
        );
    }
}

#[cfg(test)]
mod benches {
    extern crate test;
    use test::Bencher;

    use crate::{solve, wrap_line};

    #[bench]
    // Only because it takes a reasonable amount of time
    fn twelve_lines(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(
                include_str!("../../input")
                    .lines()
                    .take(12)
                    .map(wrap_line)
                    .sum::<u16>(),
                5
            )
        });
    }

    #[bench]
    fn example_1(b: &mut Bencher) {
        #[allow(clippy::iter_nth_zero)]
        b.iter(|| solve(include_str!("../../input").lines().nth(0).unwrap()));
    }

    #[bench]
    fn example_2(b: &mut Bencher) {
        b.iter(|| solve(include_str!("../../input").lines().nth(2).unwrap()));
    }

    #[bench]
    fn example_3(b: &mut Bencher) {
        b.iter(|| solve(include_str!("../../input").lines().nth(2).unwrap()));
    }
}
