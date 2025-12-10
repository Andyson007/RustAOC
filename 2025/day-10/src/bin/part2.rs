use core::fmt::Debug;
use core::iter;
use std::collections::HashMap;
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
        .inspect(|x| println!("{x}"))
        .sum()
}

pub fn solve_line(grid: Required, buttons: &[Button]) -> usize {
    // println!("{grid:?}");
    // println!("{buttons:?}");
    // returns all buttons which modify the current button and only future ones
    let get_modifiers = |led| -> Vec<Vec<usize>> {
        buttons
            .iter()
            .filter(|button| button.0.contains(&led))
            .filter(|button| button.0.iter().all(|x| *x >= led))
            .map(|x| x.0.clone())
            .collect()
    };
    let mut visited = HashMap::new();
    solve_step(0, &grid, get_modifiers, &mut visited).unwrap()
}

fn solve_step(
    curr: usize,
    grid: &Required,
    get_modifiers: impl Fn(usize) -> Vec<Vec<usize>> + Copy,
    visited: &mut HashMap<Required, usize>,
) -> Option<usize> {
    if let Some(x) = visited.get(grid) {
        return Some(*x);
    }
    if grid.is_zero() {
        return Some(0);
    }
    if grid.0[curr] == 0 {
        return solve_step(curr + 1, grid, get_modifiers, visited);
    }
    let mut min = None;
    let available = &get_modifiers(curr);
    'outer: for attempt in Combinations::new(available, grid.0[curr]) {
        let mut grid = grid.clone();
        for (b, amount) in attempt {
            if !grid.sub(b, amount) {
                continue 'outer;
            }
        }
        if let Some(s) = solve_step(curr + 1, &grid, get_modifiers, visited)
            && min.is_none_or(|min| s < min)
        {
            visited.insert(grid, s);
            min = Some(s)
        }
    }
    min.map(|x| x + grid.0[curr])
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
pub struct Required(Vec<usize>);

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
    pub fn sub(&mut self, button: &[usize], amount: usize) -> bool {
        for b in button {
            if self.0[*b] < amount {
                return false;
            }
            self.0[*b] -= amount;
        }
        true
    }
}

pub struct Combinations<'a, T> {
    arr: &'a [T],
    lens: Vec<usize>,
    needed: usize,
    done: bool,
}

impl<'a, T> Combinations<'a, T> {
    pub fn new(arr: &'a [T], amount: usize) -> Self {
        if arr.is_empty() {
            Self {
                arr,
                lens: Vec::new(),
                needed: amount,
                done: true,
            }
        } else {
            Self {
                arr,
                lens: iter::repeat_n(0, arr.iter().len() - 1).collect(),
                needed: amount,
                done: false,
            }
        }
    }
}

impl<'a, T> Iterator for Combinations<'a, T>
where
    T: Debug + Eq + Hash,
{
    type Item = HashMap<&'a T, usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.lens.is_empty() {
            self.done = true;
            return Some(HashMap::from([(&self.arr[0], self.needed)]));
        }
        let rest = self.needed - self.lens.iter().sum::<usize>();
        let ret = Some(
            self.lens
                .iter()
                .enumerate()
                .map(|(pos, amount)| (*amount, &self.arr[pos]))
                .chain(iter::once((rest, self.arr.last().unwrap())))
                .fold(
                    HashMap::with_capacity(self.arr.len()),
                    |mut acc, (amount, elem)| {
                        *acc.entry(elem).or_default() += amount;
                        acc
                    },
                ),
        );
        self.lens[0] += 1;
        let mut pos = 0;
        while self.lens.iter().sum::<usize>() > self.needed {
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

    use crate::{Button, Required, solve, solve_line};

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
        let ans = solve_line(req, &buttons);
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
        let ans = solve_line(req, &buttons);
        assert_eq!(ans, 12)
    }

    #[test]
    fn input_five() {
        solve(
            "[.#.##.#.] (6,7) (0,2) (0,3,4,5,6,7) (0,1,2,3) (2) (1,2,4,5,7) (2,4,5,6,7) (0,1,3,5,7) {47,36,44,43,19,31,162,180}",
        );
    }
}
