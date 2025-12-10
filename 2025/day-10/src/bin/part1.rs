use core::fmt::Debug;

fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

pub fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut ws = line.split_whitespace();
            let grid = Grid::parse(ws.next().unwrap());
            let buttons: Vec<_> = ws
                .take_while(|x| !x.starts_with('{'))
                .map(|x| Button::parse(x, grid.len as u128))
                .collect();
            for i in 0..buttons.len() {
                let choices = choose(i, &buttons);
                for choice in choices {
                    assert_eq!(choice.len(), i);
                    let result = choice
                        .iter()
                        .map(|x| x.buttons)
                        .reduce(core::ops::BitXor::bitxor)
                        .unwrap_or(0);
                    if result ^ grid.grid == 0 {
                        return i;
                    }
                }
            }

            unreachable!()
        })
        .sum()
}

fn choose<T: Debug>(amount: usize, list: &[T]) -> Vec<Vec<&T>> {
    if amount == 0 {
        return Vec::from([Vec::new()]);
    }
    if amount > list.len() {
        return Vec::new();
    }
    let mut ret = Vec::new();
    for i in 0..list.len() - amount + 1{
        let (curr, rest) = list[i..].split_at(1);
        let first = &curr[0];
        let rest = choose(amount - 1, rest);
        for mut variant in rest {
            variant.push(first);
            ret.push(variant);
        }
    }
    ret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Button {
    buttons: u128,
}
impl Button {
    pub fn parse(buttons: &str, len: u128) -> Self {
        let sub = buttons.trim_start_matches('(').trim_end_matches(')');
        Self {
            buttons: sub
                .split(',')
                .map(|x| x.trim().parse::<u128>().unwrap())
                .map(|x| len - x - 1)
                .map(|x| 1 << x)
                .reduce(core::ops::BitXor::bitxor)
                .unwrap(),
        }
    }
}

struct Grid {
    /// Stores what needs to be toggled
    grid: u128,
    len: usize,
}

impl Grid {
    pub fn parse(grid: &str) -> Self {
        let sub = grid.trim_start_matches('[').trim_end_matches(']');
        Self {
            grid: sub
                .chars()
                .map(|x| if x == '#' { 1 } else { 0 })
                .fold(0, |acc, curr| (acc << 1) + curr),
            len: sub.len(),
        }
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
        assert_eq!(ans, 7);
    }
}
