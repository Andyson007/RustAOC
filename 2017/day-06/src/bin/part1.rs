use std::collections::HashSet;

fn main() {
    let mut input = vec![4, 10, 4, 1, 8, 4, 9, 14, 5, 1, 14, 15, 0, 15, 3, 5];
    // let mut input = vec![0, 2, 7, 0];
    let len = input.len();
    let mut visited = HashSet::new();
    let mut count = 0;
    while !visited.contains(&input) {
        visited.insert(input.clone());
        let max = *input.iter().max().unwrap();
        let index = input.iter().position(|x| *x == max).unwrap();
        input[index] = 0;
        for i in 1..=max {
            input[(i + index) % len] += 1;
        }
        count += 1;
    }
    println!("{count}");
}
