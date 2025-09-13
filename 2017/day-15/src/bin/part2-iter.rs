use day_15::Generator;

fn main() {
    let a_iter = Generator::new(618, 16807);
    let b_iter = Generator::new(814, 48271);
    let count = a_iter
        .filter(|x| x % 4 == 0)
        .zip(b_iter.filter(|x| x % 8 == 0))
        .take(5_000_000)
        .filter(|(a, b)| (a ^ b) & 0xffff == 0)
        .count();
    println!("{count}");
}
