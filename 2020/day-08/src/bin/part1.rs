use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let mut s = line.split_whitespace();
            (
                s.next().unwrap().to_string(),
                s.next().unwrap().parse::<i16>().unwrap(),
            )
        })
        .collect::<Vec<(String, i16)>>();
    let mut acc = 0;
    let mut pos: usize = 0;
    let mut visited: HashSet<usize> = HashSet::new();
    while !visited.contains(&pos) {
        visited.insert(pos);
        match input[pos].0.as_str() {
            "nop" => pos += 1,
            "acc" => {
                acc += input[pos].1;
                pos += 1;
            }
            "jmp" => pos = (pos as i16 + input[pos].1) as usize,
            _ => unreachable!(),
        }
    }
    println!("{acc}");
}
