fn main() {
    let input = 301;
    let mut buffer = vec![0];
    let mut current = 0;
    for i in 1..=2017 {
        current = (current + input + 1) % buffer.len();
        buffer.insert(current, i);
        // println!("{buffer:?}");
    }
    println!("{}", buffer[current + 1]);
}
