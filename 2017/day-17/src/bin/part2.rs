fn main() {
    let input = 301;
    let mut current = 0;
    let mut ans = 0;
    for i in 1..=50000000 {
        current = (current + input + 1) % i;
        if current == 0 {
          ans = i;
        }
    }
    println!("{ans}");
}
