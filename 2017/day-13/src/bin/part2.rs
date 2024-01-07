use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let split = line
                .split(": ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (split[0], split[1])
        })
        .collect::<HashSet<(usize, usize)>>();
    for delay in 0.. {
        if !input.iter().any(|(a, b)| (a + delay) % (2 * b - 2) == 0) {
            println!("{delay}");
            break;
        }
    }
}
