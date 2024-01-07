use std::collections::{HashMap, HashSet};

fn main() {
    let maps = include_str!("../../input.txt")
        .lines()
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
    let mut santa = find("COM", "SAN", &maps).unwrap();
    let mut you = find("COM", "YOU", &maps).unwrap();
    while santa.last()==you.last() {
      santa.pop();
      you.pop();
    }
    println!("{}", santa.len()+you.len());
}

fn find(start: &str, target: &str, maps: &HashMap<&str, HashSet<&str>>) -> Option<Vec<String>> {
    if start == target {
        // Some(Vec::from([start.to_string()]))
      Some(Vec::new())
    } else {
        match maps.get(start)
            .unwrap_or(&&HashSet::new())
            .iter()
            .filter_map(|x| find(x, target, &maps))
            .nth(0) {
              None => None,
              Some(mut x) => {
                x.push(start.to_string());
                Some(x)
              }
            }
    }
}
