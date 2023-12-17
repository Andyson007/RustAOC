fn main() {
    let mut lines = include_str!("../../input.txt")
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    lines.sort();
    lines.insert(0, 0);
    let diffs = lines
        .windows(2)
        .map(|arr| arr[1] - arr[0])
        .collect::<Vec<u32>>();
    let ans = diffs.iter().fold((0, 1), |sum, curr| {
        (
            sum.0
                + match curr {
                    1 => 1,
                    _ => 0,
                },
            sum.1
                + match curr {
                    3 => 1,
                    _ => 0,
                },
        )
    });
    print!("{}", ans.0 * ans.1);
}
