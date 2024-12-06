use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    let (rules, prints) = raw.split_once("\n\n").unwrap();
    let rules = {
        let rules = rules
            .lines()
            .map(|x| {
                let (l, r) = x.split_once('|').unwrap();
                let l = l.parse::<usize>().unwrap();
                let r = r.parse::<usize>().unwrap();
                (r, l)
            })
            .collect::<Vec<_>>();
        let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
        for elem in rules {
            if let Some(x) = map.get_mut(&elem.0) {
                x.insert(elem.1);
            } else {
                map.insert(elem.0, HashSet::from([elem.1]));
            }
        }
        map
    };
    // .collect::<HashMap<usize, usize>>();
    prints
        .lines()
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|x| {
            for (i, curr) in x.iter().enumerate() {
                let rules_to_find = rules.get(curr).unwrap_or(&HashSet::new()).clone();
                let mut to_find = rules_to_find
                    .intersection(&HashSet::from_iter(x[i..].iter().copied()))
                    .copied()
                    .collect::<HashSet<usize>>();
                for elem in x.iter().take(i) {
                    if to_find.contains(elem) {
                        to_find.remove(elem);
                    }
                }
                if !to_find.is_empty() {
                    return false;
                }
            }
            true
        })
        .map(|x| {
            let mid = (x.len() - 1) / 2;
            x[mid]
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 143)
    }
}
