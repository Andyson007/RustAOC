fn main() {
    let ans = include_str!("../../input.txt")
        .split("\r\n\r\n")
        .map(|elf| {
            elf.lines()
                .map(|line| {line.trim().parse::<u32>().unwrap()})
                .sum::<u32>()
        }).max().unwrap();
    println!("{ans}");
}
