use std::collections::HashMap;

fn main() {
    let mut prevs = [16u64, 12u64, 1u64, 0u64, 15u64, 7u64, 11u64]
        // let mut prevs = [0, 3, 6]
        .iter()
        .enumerate()
        .map(|(i, v)| (*v as usize, i + 1))
        .collect::<HashMap<usize, usize>>();
    let mut prev = 0;
    let len = 30000000;
    for i in prevs.len() + 1..30000000 {
        if i % 1000000 == 0 {
            println!("{:2.5}%", 100f64 * i as f64 / len as f64);
        }
        if let Some(&x) = prevs.get(&prev) {
            prevs.insert(prev, i);
            prev = i - x;
        } else {
            prevs.insert(prev, i);
            prev = 0;
        }
    }
    println!("{prev}");
}
