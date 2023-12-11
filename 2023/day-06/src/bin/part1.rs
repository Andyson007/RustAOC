fn main() {
    let input = include_str!("../../input.txt");
    let ans = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|str| str.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .iter()
        .zip(
            input
                .lines()
                .nth(1)
                .unwrap()
                .split_whitespace()
                .skip(1)
                .map(|str| str.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        )
        .map(|x| (*x.0, x.1))
        .map(|(time, max)| {
            let mut count = 0;
            for i in 0..=time {
                if i * (time - i) > max {
                    count += 1;
                }
            }
            count
        })
        .fold(1u32, |sum, curr| sum * curr);
    println!("{ans}");
}
