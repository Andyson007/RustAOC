use std::{
    collections::{BinaryHeap, HashSet},
    iter,
};

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    let first_line = raw
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .enumerate()
        .filter(|x| x.1 != 0)
        .map(|(id, elem)| {
            if id & 1 == 0 {
                (Some(id / 2), elem)
            } else {
                (None, elem)
            }
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    let mut pos = 0;
    let mut todo = first_line
        .iter()
        .filter_map(|x| x.0.map(|y| (y, x.1)))
        .rev()
        .collect::<Vec<_>>();
    let mut forwards = first_line.iter();
    while !todo.is_empty() {
        let (id, mut amount) = forwards.next().unwrap();
        if let Some(id) = id {
            if let Some(curr_pos) = todo.iter().position(|x| x.0 == *id) {
                todo.remove(curr_pos);
                for i in 0..amount {
                    sum += (pos + i) * id;
                }
            }
        } else {
            while let Some(found) = todo.iter().find(|x| x.1 <= amount).copied() {
                let curr_pos = todo.iter().position(|x| x.0 == found.0);

                todo.remove(curr_pos.unwrap());
                let len = found.1;
                amount -= len;

                for i in 0..len {
                    sum += (pos + i) * found.0;
                }
                pos += len;
            }
        }
        pos += amount;
    }
    sum
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 2858);
    }
}
