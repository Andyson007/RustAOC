use day_15::Generator;

fn main() {
    let a_iter = Generator::new(618, 16807);
    let b_iter = Generator::new(814, 48271);
    let count = a_iter
        .zip(b_iter)
        .take(40000000)
        .filter(|(a, b)| (a ^ b) & 0xffff == 0)
        .count();
    println!("{count}");
}
