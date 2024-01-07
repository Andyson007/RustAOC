use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let name = line.split_whitespace().nth(0).unwrap();
            let val1 = line
                .split_whitespace()
                .nth(3)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let val2 = line
                .split_whitespace()
                .nth(6)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let val3 = line
                .split_whitespace()
                .nth(13)
                .unwrap()
                .parse::<u32>()
                .unwrap();
            (name, (val1, val2, val3))
        })
        .collect::<HashMap<&str, (u32, u32, u32)>>();
    let mut distances = input
        .iter()
        .map(|(k, _v)| (*k, 0))
        .collect::<HashMap<&str, u32>>();
    let mut scores = distances.clone();
    for i in 0..2503 {
        for (k, v) in distances.iter_mut() {
            let vals = input.get(k).unwrap();
            if i % (vals.1 + vals.2) < vals.1 {
                *v += vals.0;
            }
        }
        let max = distances.iter().map(|(_k, v)| v).max().unwrap();
        for (k, v) in &distances {
          if v == max {
            *scores.get_mut(k).unwrap() += 1;
          }
        }
    }
    for a in scores {
        println!("{a:?}");
    }
}
