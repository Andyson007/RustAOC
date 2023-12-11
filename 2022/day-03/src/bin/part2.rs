fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| String::from(line))
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|arr| {
            let (a, b, c) = (
                arr.iter().nth(0).unwrap(),
                arr.iter().nth(1).unwrap(),
                arr.iter().nth(2).unwrap(),
            );
            for item in a.chars() {
                if b.matches(item).count() == 0 {
                    continue;
                }
                if c.matches(item).count() == 0 {
                    continue;
                }
                return item;
            }
            unreachable!();
        })
        .map(|item| {
            let item = (item as u8) - 'A' as u8 + 1;
            return ((item & !32) + if (item & 32) == 0 { 26 } else { 0 }) as u32;
        })
        .sum::<u32>();
    println!("{ans}");
}
