use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash)]
struct Insturction {
    requirement: char,
    part: char,
}

impl Ord for Insturction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.part.cmp(&self.part)
    }
}

fn main() {
    let input = include_str!("../../input.txt")        .lines()
        .map(|x| {
            let split = x.split(" ").collect::<Vec<&str>>();
            (
                split[1].chars().nth(0).unwrap(),
                split[7].chars().nth(0).unwrap(),
            )
        })
        .collect::<HashSet<(char, char)>>();
    let parts = input.iter().fold(HashSet::new(), |mut sum, curr| {
        sum.insert(curr.0);
        sum.insert(curr.1);
        sum
    });
    let mut counts = input.iter().fold(HashMap::new(), |mut sum, curr| {
        if !sum.contains_key(&curr.1) {
            sum.insert(curr.1, 0);
        }
        *sum.get_mut(&curr.1).unwrap() += 1;
        sum
    });
    println!("{counts:?}");
    let input = input.iter().fold(HashMap::new(), |mut sum, curr| {
        if !sum.contains_key(&curr.0) {
            sum.insert(curr.0, HashSet::new());
        }
        sum.get_mut(&curr.0).unwrap().insert(curr.1);
        sum
    });
    let mut heap = BinaryHeap::new();
    for i in parts.iter().filter(|x| !counts.contains_key(x)).map(|x| *x) {
        heap.push(Reverse(i));
    }
    while let Some(x) = heap.pop() {
        let x = x;
        if let Some(x) = input.get(&x.0) {
            for val in x {
                if let Some(x) = counts.get_mut(val) {
                    *x -= 1;
                    if *x == 0 {
                        heap.push(Reverse(*val));
                    }
                }
            }
        }
        print!("{}", x.0);
    }
}
