use std::collections::HashSet;

fn main() {
    let ans = include_str!("../../input.txt")
        .split("\r\n\r\n")
        .map(|group| {
            let mut answers = group
                .lines()
                .map(|line| line.chars().collect::<Vec<char>>());

            let mut ret: HashSet<char> = HashSet::from_iter(answers.next().unwrap());
            for a in answers {
              let mut newset: HashSet<char> = HashSet::new();
              for t in a {
                if ret.contains(&t) {
                    newset.insert(t);
                }
              }
              ret = newset;
            }
            ret.iter().count() as u32
        })
        .sum::<u32>();
    println!("{ans}");
}
