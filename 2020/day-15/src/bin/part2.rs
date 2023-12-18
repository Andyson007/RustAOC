use std::collections::HashMap;

fn main() {
    let vals: Vec<usize> = vec![0, 3, 6];
    let mut values: HashMap<usize, usize> = HashMap::new();
    for (index, value) in vals.iter().enumerate() {
        values.insert(*value, index);
    }
    let length = vals.len();
    let mut prev = 0;
    for i in length..=(2020 + length) {
        let dist = if let Some(x) = values.get_mut(&prev) {
            let var = i - *x;
            *x = i;
            var
        } else {
            values.insert(prev, i-1);
            0
        };
        prev = dist;
        println!("{prev}");
    }
}
