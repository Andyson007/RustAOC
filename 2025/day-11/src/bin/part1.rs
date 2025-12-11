use std::collections::HashMap;

fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

pub fn solve(input: &str) -> usize {
    let data: HashMap<&str, Vec<&str>> = input.lines().map(Node::parse).rfold(HashMap::new(), |mut acc, curr| {
        acc.entry(curr.name).or_default().extend_from_slice(&curr.dests);
        acc
    });
    let mut poses = HashMap::from([("you", 1)]);
    let mut total = 0;
    while !poses.is_empty() {
        let mut new = HashMap::new();
        for (pos, amount) in poses {
            if pos == "out" {
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
            r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
        );
        assert_eq!(ans, 5)
    }
}
