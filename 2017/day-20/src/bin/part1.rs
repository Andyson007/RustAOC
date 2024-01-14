fn main() {
    let ans = include_str!("../../input.txt")
        .lines().enumerate()
        .map(|(i, line)| {
            (i, line.split(", ")
                .nth(2)
                .unwrap()
                .split(",")
                .map(|s| {
                    s.chars()
                        .skip_while(|x| !x.is_digit(10))
                        .take_while(|x| x.is_digit(10))
                        .collect::<String>()
                        .parse::<i64>()
                        .unwrap()
                        .abs_diff(0) as u64
                })
                .sum::<u64>())
        })
        .min_by(|a,b| a.1.cmp(&b.1))
        .unwrap().0;
    println!("{ans}");
}
