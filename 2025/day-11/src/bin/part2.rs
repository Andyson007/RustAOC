#![feature(test)]
use std::collections::HashMap;

fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

pub fn solve(input: &str) -> usize {
    let data: HashMap<&str, Vec<&str>> =
        input
            .lines()
            .map(Node::parse)
            .rfold(HashMap::new(), |mut acc, curr| {
                acc.entry(curr.name)
                    .or_default()
                    .extend_from_slice(&curr.dests);
                acc
            });
    (paths("svr", "fft", &data)) * paths("fft", "dac", &data) * paths("dac", "out", &data)
        + paths("svr", "dac", &data) * paths("dac", "fft", &data) * paths("fft", "out", &data)
}

fn paths(src: &str, dst: &str, data: &HashMap<&str, Vec<&str>>) -> usize {
    let mut poses = HashMap::from([(src, 1)]);
    let mut total = 0;
    while !poses.is_empty() {
        let mut new = HashMap::with_capacity(data.len());
        for (pos, amount) in poses {
            if pos == dst {
                total += amount;
                continue;
            }
            let Some(edges) = data.get(pos) else {
                continue;
            };

            for next in edges {
                *new.entry(*next).or_default() += amount;
            }
        }
        poses = new;
    }
    total
}

struct Node<'a> {
    name: &'a str,
    dests: Vec<&'a str>,
}

impl<'a> Node<'a> {
    pub fn parse(data: &'a str) -> Self {
        let name = &data[0..3];
        let rest = &data[5..];
        let amount = (rest.len() + 1) / 4;
        let mut dests = Vec::with_capacity(amount);
        for i in 0..amount {
            dests.push(&rest[4 * i..4 * i + 3]);
        }
        Self { name, dests }
    }
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let ans = solve(
            r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
        );
        assert_eq!(ans, 2)
    }
}

#[cfg(test)]
mod bench {
    extern crate test;
    use test::Bencher;

    use crate::solve;
    #[bench]
    fn full(b: &mut Bencher) {
        b.iter(|| solve(include_str!("../../input")));
    }
}
