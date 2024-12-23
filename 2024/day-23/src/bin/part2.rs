use std::{collections::HashSet, iter};

use itertools::Itertools;

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}")
}

fn solve(raw: &str) -> String {
    let mut networks: Vec<HashSet<&str>> = Vec::new();
    for (l, r) in raw.lines().map(|line| line.split_once('-').unwrap()) {
        if let (Some(l_pos), Some(r_pos)) = (
            networks.iter().position(|x| x.contains(l)),
            networks.iter().position(|x| x.contains(r)),
        ) {
            if l_pos != r_pos {
                let left_net = networks.swap_remove(l_pos);
                let right_net = networks.swap_remove(if r_pos > l_pos { r_pos - 1 } else { r_pos });

                let to_push = left_net.union(&right_net).copied().collect();
                networks.push(to_push);
            }
        } else if let Some(x) = networks.iter_mut().find(|x| x.contains(&l)) {
            x.insert(r);
        } else if let Some(x) = networks.iter_mut().find(|x| x.contains(&r)) {
            x.insert(l);
        } else {
            networks.push(HashSet::from([l, r]))
        }
    }

    let mut largest_nets = networks
        .into_iter()
        .map(|network| solve_network(network, raw))
        .map(|x| {
            let mut vec = x.into_iter().collect::<Vec<_>>();
            vec.sort();
            vec
        })
        .collect::<Vec<_>>();

    largest_nets.sort_by_key(|x| x.len());

    for net in &largest_nets {
        println!("{net:?}");
    }

    largest_nets
        .last()
        .unwrap()
        .iter()
        .cloned()
        .interleave(iter::repeat_n(
            ",".to_string(),
            largest_nets.last().unwrap().len() - 1,
        ))
        .collect::<String>()
}

fn solve_network(network: HashSet<&str>, raw: &'_ str) -> HashSet<String> {
    let mut edges: HashSet<(&str, &str)> = HashSet::new();
    for (l, r) in raw.lines().map(|line| line.split_once('-').unwrap()) {
        if network.contains(l) && network.contains(r) {
            edges.insert((l, r));
        }
    }
    let mut longest = HashSet::new();
    'outer: for i in 1.. {
        println!("{i}");
        for node in network.iter() {
            'inner: for to_check in network
                .iter()
                .filter(|x| edges.contains(&(*node, **x)) || edges.contains(&(**x, node)))
                .combinations(i)
            {
                for other in &to_check {
                    for other2 in &to_check {
                        if other == other2 {
                            continue;
                        }
                        if !(edges.contains(&(other, other2)) || edges.contains(&(other2, other))) {
                            continue 'inner;
                        }
                    }
                }
                longest = to_check
                    .into_iter()
                    .chain(iter::once(node))
                    .map(|x| x.to_string())
                    .collect();
                continue 'outer;
            }
            if longest.len() != i {
                break 'outer;
            }
        }
    }
    longest
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        assert_eq!(solve(include_str!("../../example")), "co,de,ka,ta");
    }
}
