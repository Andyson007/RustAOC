fn main() {
    let input = 3012210;
    let mut prev = 1;
    for n in 3..=input {
        prev = if prev < n / 2 { prev + 1 } else { prev + 2 } % n;
    }
    println!("{prev}");
}
