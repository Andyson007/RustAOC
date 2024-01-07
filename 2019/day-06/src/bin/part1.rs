use std::collections::{HashMap, HashSet};

fn main() {
    let maps = include_str!("../../input.txt")    .lines()
    .map(|line| line.split(")").collect::<Vec<&str>>())
    .map(|arr| (arr[0], arr[1]))
    .fold(
        HashMap::new(),
        |mut sum: HashMap<&str, HashSet<&str>>, curr| {
            if let Some(x) = sum.get_mut(curr.0) {
                x.insert(curr.1);
            } else {
                sum.insert(curr.0, HashSet::from([curr.1]));
            }
            sum
        },
    );
    let mut curr = HashSet::from(["COM"]);
    let mut total = 0;
    for dist in 0.. {
        let mut new = HashSet::new();
        for c in curr {
          total+=dist;
            for v in maps.get(&c).unwrap_or(&HashSet::new()) {
                new.insert(*v);
            }
        }
        curr = new;
        if curr.len() == 0 {
          break;
        }
    }
    println!("{total}");
}
