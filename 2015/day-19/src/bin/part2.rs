use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    let mut input = include_str!("../../input.txt").split("\r\n\r\n");
    let mappings = input
        .next()
        .unwrap()
        .lines()
        .rev()
        .map(|line| {
            let line = line.split(" => ").collect::<Vec<&str>>();
            (line[1], line[0])
        })
        .collect::<HashSet<(&str, &str)>>();
    let molecule = input.next().unwrap().lines().nth(0).unwrap();
    let ans = solve(molecule.to_string(), &mappings);
    println!("{ans:?}");
}

fn solve(molecule: String, mappings: &HashSet<(&str, &str)>) -> Option<usize> {
    let mut heap = BinaryHeap::new();

    heap.push(State {
        content: molecule.clone(),
        cost: 0,
    });

    while let Some(State { content, cost }) = heap.pop() {
        // println!("{}",content.len());
        if content == "e" {
            return Some(cost);
        }
        for opt in options(content, mappings) {
            if !contains(heap.clone(), &opt) {
                heap.push(State {
                    content: opt.to_string(),
                    cost: cost + 1,
                })
            }
        }
    }
    None
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    content: String,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.content.len().cmp(&self.content.len())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn options(molecule: String, mappings: &HashSet<(&str, &str)>) -> HashSet<String> {
    let mut molecules: HashSet<String> = HashSet::new();
    for map in mappings {
        let split = molecule.split(map.0).collect::<Vec<&str>>();
        for i in 1..split.len() {
            let mut new = split
                .iter()
                .take(i)
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(map.0);
            new.push_str(map.1);
            new.push_str(
                split
                    .iter()
                    .skip(i)
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(map.0)
                    .as_str(),
            );
            molecules.insert(new.clone());
        }
    }
    molecules
}

fn contains(heap: BinaryHeap<State>, needle: &String) -> bool {
    let vec = heap
        .into_sorted_vec()
        .into_iter()
        .map(|x| x.content)
        .collect::<Vec<String>>();
    let mut start = 0;
    let mut end = vec.len();
    let mut middle = (start + end) / 2;
    while start != middle {
        match vec[middle].len().cmp(&needle.len()) {
            Ordering::Less => start = middle,
            Ordering::Greater | Ordering::Equal => end = middle,
        }
        middle = (start + end) / 2;
    }
    for i in start..vec.len() {
      if vec[i] == *needle {
        return true;
      }
      if vec[i].len()!=needle.len() {
        break;
      }
    }
    false
}
