use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}")
}

fn solve(raw: &str) -> usize {
    let mut nodes = HashSet::new();
    let mut edges = HashSet::new();
    for (l, r) in raw.lines().map(|line| line.split_once('-').unwrap()) {
        nodes.insert(l);
        nodes.insert(r);
        edges.insert((l, r));
        edges.insert((r, l));
    }
    let mut triple_nets = HashSet::new();
    for nodes in nodes.into_iter().combinations(3) {
        if edges.contains(&(nodes[0], nodes[1]))
            && edges.contains(&(nodes[1], nodes[2]))
            && edges.contains(&(nodes[0], nodes[2]))
        {
            triple_nets.insert(nodes);
        }
    }

    triple_nets
        .into_iter()
        .filter(|x| !x.iter().all(|x| !x.starts_with('t')))
        .count()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        assert_eq!(solve(include_str!("../../example")), 7);
    }
}
