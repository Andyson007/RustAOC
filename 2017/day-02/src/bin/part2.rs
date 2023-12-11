fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let line = line.split_whitespace().map(|x| x.parse::<u32>().unwrap());
            for a in line.clone().enumerate() {
                for b in line.clone().enumerate() {
                    if a.0 == b.0 {
                        continue;
                    }
                    if b.1 % a.1 == 0 {
                        return b.1 / a.1;
                    }
                }
            }
            unreachable!();
        })
        .sum::<u32>();
    println!("{ans}");
}
