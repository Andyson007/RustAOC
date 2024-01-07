fn main() {
    let mut register: i32 = 1;
    let mut cycle: i32 = 0;
    let mut display = [false; 40 * 6];
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| line.to_string().trim().to_string())
        .collect::<Vec<String>>();
    for line in input {
        if line == "noop" {
            display[cycle as usize] = register.abs_diff(cycle % 40) <= 1;
            cycle += 1;
        } else {
            let val = line
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<i32>()
                .unwrap();
            display[cycle as usize] = register.abs_diff(cycle % 40) <= 1;
            cycle += 1;
            display[cycle as usize] = register.abs_diff(cycle % 40) <= 1;
            cycle += 1;
            register += val;
        }
    }
    for row in display.chunks(40) {
        println!(
            "{}",
            row.iter()
                .map(|x| if *x { '#' } else { ' ' })
                .collect::<String>()
        )
    }
}
