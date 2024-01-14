fn main() {
    let mut vals: Vec<usize> = vec![16, 12, 1, 0, 15, 7, 11];
    while vals.len() < 2020 {
        let value = vals[vals.len() - 1];
        let pos = vals.iter().rev().skip(1).position(|x| *x == value);
        if let Some(pos) = pos {
            vals.push(pos + 1);
        } else {
            vals.push(0);
        }
    }
    println!("{}", vals[vals.len() - 1]);
}
