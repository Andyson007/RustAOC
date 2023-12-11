use std::collections::HashMap;

fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| {
                    x.chars().fold(HashMap::new(), |mut sum, curr| {
                        if let Some(y) = sum.get_mut(&curr) {
                            *y += 1;
                        } else {
                            sum.insert(curr, 1);
                        }
                        sum
                    })
                })
                .collect::<Vec<HashMap<char, u8>>>()
        })
        .filter(|line| {
            for i in 0..line.len() {
                for j in (i + 1)..line.len() {
                    if line[i] == line[j] {
                        return false;
                    }
                }
            }
            true
        })
        .count();
    println!("{ans}");
}
