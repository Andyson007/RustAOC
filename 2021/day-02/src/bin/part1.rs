fn main() {
    let pos = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let (dir, count) = (
                line.trim().split_whitespace().nth(0).unwrap(),
                line.trim()
                    .split_whitespace()
                    .nth(1)
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
            );
            match dir {
                "forward" => (count, 0),
                "down" => (0, count),
                "up" => (0, -count),
                _ => unreachable!(),
            }
        })
        .fold((0, 0), |sum, curr| (sum.0 + curr.0, sum.1 + curr.1));
    let ans = pos.0 * pos.1;
    println!("{ans}");
}
