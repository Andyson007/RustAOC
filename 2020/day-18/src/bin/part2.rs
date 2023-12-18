use std::collections::HashMap;

fn main() {
    let order = HashMap::from([('(', 0), ('+', 2), ('*', 1)]);
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let mut stack: Vec<char> = Vec::new();
            let mut ops: Vec<char> = Vec::new();
            for c in line.chars().filter(|c| !c.is_whitespace()) {
                if c.is_digit(10) {
                    stack.push(c);
                } else if c == '(' {
                    ops.push('(');
                } else if c == ')' {
                    while ops.len() != 0 && ops[ops.len() - 1] != '(' {
                        stack.push(ops[ops.len() - 1]);
                        ops.pop();
                    }
                    ops.pop();
                } else {
                    if ops.len() == 0
                        || *order.get(&ops[ops.len() - 1]).unwrap() < *order.get(&c).unwrap()
                    {
                        ops.push(c);
                    } else {
                        stack.push(ops[ops.len() - 1]);
                        ops.pop();
                        ops.push(c);
                    }
                }
            }
            while ops.len() != 0 {
                stack.push(ops[ops.len() - 1]);
                ops.pop();
            }
            let mut values: Vec<u64> = Vec::new();
            for c in stack {
                if c.is_digit(10) {
                    values.push(c.to_digit(10).unwrap() as u64);
                } else {
                    let val1 = values.last().unwrap().clone();
                    values.pop();
                    let val2 = values.last().unwrap().clone();
                    values.pop();
                    values.push(match c {
                        '*' => val1 * val2,
                        '+' => val1 + val2,
                        _ => unreachable!(),
                    })
                }
            }
            values[0]
        })
        .sum::<u64>();
    println!("{ans}");
}
