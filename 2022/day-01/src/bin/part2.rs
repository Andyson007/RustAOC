fn main() {
    let mut ans = include_str!("../../input.txt")
        .split("\r\n\r\n")
        .map(|elf| {
            elf.lines()
                .map(|line| line.trim().parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    ans.sort();
    ans.reverse();
    println!("{}", ans.iter().take(3).sum::<u32>())
}
