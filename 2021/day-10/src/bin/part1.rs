fn main() {
    let ans = include_str!("../../input.txt")
    .lines()
    .filter_map(|line| {
        let mut brackets = Vec::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '<' | '{' => brackets.push(c),
                ')' => {
                    if brackets.remove(brackets.len() - 1) != '(' {
                        return Some(3);
                    }
                }
                ']' => {
                    if brackets.remove(brackets.len() - 1) != '[' {
                        return Some(57);
                    }
                }
                '}' => {
                    if brackets.remove(brackets.len() - 1) != '{' {
                        return Some(1197);
                    }
                }
                '>' => {
                    if brackets.remove(brackets.len() - 1) != '<' {
                        return Some(25137);
                    }
                }
                _ => unreachable!(),
            }
        }
        None
    })
    .sum::<u64>();
    println!("{ans}");
}
