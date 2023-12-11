fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let length = line.len();
            line.split_at(length / 2)
        })
        .map(|(first, second)| {
            for f in first.chars() {
                let pos = second.find(f);
                if let Some(pos) = pos {
                    return second.chars().nth(pos).unwrap();
                }
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
