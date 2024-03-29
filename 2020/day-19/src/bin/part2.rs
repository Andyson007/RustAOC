use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Rule {
    Const(bool),
    Other(HashSet<Vec<u64>>),
}

fn main() {
    // let input = include_str!("../../input.txt");
    let input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let mut maps = parts[0]
        .lines()
        .map(|line| {
            let split = line.split(": ").collect::<Vec<&str>>();
            let id = split[0].parse::<u64>().unwrap();
            if split[1].chars().nth(0).unwrap() == '"' {
                (id, Rule::Const(split[1].chars().nth(1).unwrap() == 'a'))
            } else {
                let rules = split[1]
                    .split(" | ")
                    .map(|s| {
                        s.split_whitespace()
                            .map(|x| x.parse::<u64>().unwrap())
                            .collect::<Vec<u64>>()
                    })
                    .collect::<HashSet<Vec<u64>>>();
                (id, Rule::Other(rules))
            }
        })
        .collect::<HashMap<u64, Rule>>();
    *maps.get_mut(&8).unwrap() = Rule::Other(HashSet::from([vec![42], vec![42, 8]]));
    *maps.get_mut(&11).unwrap() = Rule::Other(HashSet::from([vec![42, 31], vec![42, 11, 31]]));
    let possible = solve(0, &maps, parts[1].lines().map(|x| x.len()).max().unwrap());
    let ans = parts[1]
        .lines()
        .filter(|x| possible.contains(&x.chars().map(|x| x == 'a').collect::<Vec<bool>>()))
        .count();
    println!("{ans}");
    
}

fn solve(id: u64, maps: &HashMap<u64, Rule>, maxlen: usize) -> HashSet<Vec<bool>> {
    match maps.get(&id).unwrap() {
        Rule::Const(x) => HashSet::from([vec![*x]]),
        Rule::Other(x) => {
            let mut ret = HashSet::new();
            for v in x {
                let mut pushret = solve(*v.last().unwrap(), &maps, maxlen);
                for a in v.iter().rev().skip(1) {
                    let mut new = HashSet::new();
                    'outer: for b in solve(*a, &maps, maxlen) {
                        for val in pushret.clone() {
                            if b.len() + val.len() > maxlen {
                                continue 'outer;
                            }
                            let topush = [b.clone(), val].concat();
                            new.insert(topush);
                        }
                    }
                    pushret = new;
                }
                for val in pushret {
                    ret.insert(val);
                }
            }
            ret
        }
    }
}
