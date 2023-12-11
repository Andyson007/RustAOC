fn main() {
    let mut ages = include_str!("../../input.txt")
        .split(",")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .fold([0u64; 9], |mut sum, curr| {
            sum[curr] += 1;
            sum
        });
    println!();
    for _ in 0..256 {
        let mut next = [0u64; 9];
        for (i, amount) in ages.iter().enumerate() {
            if i == 0 {
                next[8] += amount;
                next[6] += amount;
            } else {
                next[i - 1] += amount;
            }
        }
        ages = next;
    }
    let ans = ages.iter().sum::<u64>();
    println!("{ans}")
}
