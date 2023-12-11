fn main() {
    let mut sum = 0;
    let mut register = 1;
    let mut cycle = 1;
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| line.to_string().trim().to_string())
        .collect::<Vec<String>>();
    for line in input {
        if (cycle + 20) % 40 == 0 {
            sum += register*cycle;
        }
        if line == "noop" {
            cycle += 1;
        } else {
            let val = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            cycle += 2;
            if (cycle + 20) % 40 == 1 {
                sum += register*(cycle-1);
            }
            register += val;
        }
    }
    println!("{sum}");
}
