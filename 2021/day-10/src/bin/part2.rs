fn main() {
    let mut lines = include_str!("../../input.txt")
        .lines()
        .filter_map(|line| {
            let mut brackets = Vec::new();
            for c in line.chars() {
                match c {
                    '(' | '[' | '<' | '{' => brackets.push(c),
                    ')' => {
                        if brackets.remove(brackets.len() - 1) != '(' {
                            return None;
                        }
                    }
                    ']' => {
                        if brackets.remove(brackets.len() - 1) != '[' {
                            return None;
                        }
                    }
                    '}' => {
                        if brackets.remove(brackets.len() - 1) != '{' {
                            return None;
                        }
                    }
                    '>' => {
                        if brackets.remove(brackets.len() - 1) != '<' {
                            return None;
                        }
                    }
                    _ => unreachable!(),
                }
            }
            let mut sum = 0;
            for bracket in brackets.iter().rev() {
                sum *= 5;
                sum += match bracket {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _=> unreachable!()
                }
            }
            Some(sum)
        }).collect::<Vec<u64>>();
    lines.sort();
    println!("{}", lines[(lines.len()-1)/2]);
}
