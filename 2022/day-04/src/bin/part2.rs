fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let matches = line
                .split(",")
                .map(|range| {
                    let vals = range
                        .split("-")
                        .map(|val| val.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
                    (vals[0], vals[1])
                })
                .collect::<Vec<(u32, u32)>>();
            (matches[0], matches[1])
        })
        .fold(0u32, |sum, ((a, b), (c, d))| {
            if (c <= a && a <= d) || (c <= b && b <= d) || (a <= c && c <= b) || (a <= d && d <= b)
            {
                return sum + 1;
            }
            sum
        });
    println!("{ans}");
}
