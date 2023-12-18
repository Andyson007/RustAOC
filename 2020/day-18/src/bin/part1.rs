fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let mut values: Vec<u64> = Vec::new();
            let mut ops: Vec<char> = Vec::new();
            values.push(0);
            ops.push('+');
            for c in line.chars().filter(|c| !c.is_whitespace()) {
                if let Some(digit) = c.to_digit(10) {
                    let other = values[values.len() - 1];
                    let lastindex = values.len() - 1;
                    values[lastindex] = match ops[ops.len() - 1] {
                        '*' => other * digit as u64,
                        '+' => other + digit as u64,
                        _ => unreachable!(),
                    };
                    ops.pop();
                } else if c == '*' || c == '+' {
                    ops.push(c);
                } else if c == '(' {
                    values.push(0);
                    ops.push('+');
                } else if c == ')' {
                    let digit = values[values.len() - 1];
                    values.pop();
                    let other = values[values.len() - 1];
                    let lastindex = values.len() - 1;
                    values[lastindex] = match ops[ops.len() - 1] {
                        '*' => other * digit,
                        '+' => other + digit,
                        _ => unreachable!(),
                    };
                    ops.pop();
                }
            }
            values[0]
        })
        .sum::<u64>();
    println!("{ans}")
}
