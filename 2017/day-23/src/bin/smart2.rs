fn main() {
    let ans = (105700..)
        .step_by(17)
        .take(1001)
        .filter(|&x| {
            for i in 2..x {
                if x % i == 0 {
                    return true;
                }
            }
            false
        })
        .count();
    println!("{ans}");
}
