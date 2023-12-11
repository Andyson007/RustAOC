fn main() {
    let commands = include_str!("../../input.txt")
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
        .collect::<Vec<(i32, i32)>>();
    let mut ans = (0, 0);
    let mut dir = 0;
    for (a, b) in commands {
        if a == 0 {
            dir += b;
        } else {
            ans.0 += a;
            ans.1 += dir * a;
        }
    }
    println!("{}", ans.0 * ans.1);
}
