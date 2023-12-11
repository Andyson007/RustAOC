fn main() {
    let input = include_str!("../input.txt");
    let ans = input
        .lines()
        .nth(0)
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|string| string.trim().parse::<u64>().unwrap())
        .map(|seed| {
            input
                .split("\r\n\r\n")
                .skip(1)
                .map(|lines| {
                    lines
                        .lines()
                        .skip(1)
                        .filter(|line| line.trim().len() != 0)
                        .map(|line| {
                            line.split(" ")
                                .take(3)
                                .map(|value| value.trim().parse::<u64>().unwrap())
                                .collect::<Vec<u64>>()
                        })
                        .collect::<Vec<Vec<u64>>>()
                })
                .collect::<Vec<Vec<Vec<u64>>>>()
                .iter()
                .fold(seed, |curr, map| {
                    match map
                        .iter()
                        .find(|line| line[1] <= curr && (line[1] + line[2]) > curr)
                    {
                        Some(line) => curr + line[0] - line[1],
                        None => curr,
                    }
                })
        })
        .min()
        .unwrap();
    println!("{ans}");
}
