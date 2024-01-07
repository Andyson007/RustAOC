fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let split = line
                .split(": ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (split[0], split[1])
        })
        .map(|(a, b)| if a % (2 * b - 2) == 0 { a * b } else { 0 })
        .sum::<usize>();
    println!("{ans}");
}
