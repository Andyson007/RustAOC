fn main() {
    let ans = include_str!("../../example.txt")
        .split(",")
        .map(|s| {
            s.trim().chars()
                .fold(0, |sum, curr| {
                    (sum + curr as u8) * 17
                })
        })
    .map(|x| x as u32)
        .sum::<u32>();
    println!("{ans}");
}
