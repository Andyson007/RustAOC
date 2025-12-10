fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

pub fn solve(input: &str) -> u64 {
    let mut iter = input.lines().rev();
    let ops = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(Op::new)
        .collect::<Vec<_>>();
    let mut lines = iter.collect::<Vec<&str>>();
    lines.reverse();
    let mut values = Vec::new();
    let mut curr = 0;
    let max_len = lines.iter().map(|x| x.len()).max().unwrap();
    for _ in 0..ops.len() {
        let mut new_vals = Vec::new();
        loop {
            let mut looped = false;
            let mut val = 0;
            for line in lines
                .iter()
                .filter_map(|line| line.chars().nth(curr).unwrap().to_digit(10))
            {
                looped = true;
                val = val * 10 + line as u64;
            }
            curr += 1;
            if looped {
                new_vals.push(val);
            } else {
                break;
            }
            if curr == max_len {
                break;
            }
        }
        values.push(new_vals);
    }
    values
        .into_iter()
        .zip(ops)
        .map(|(vals, op)| vals.into_iter().reduce(op.op()).unwrap())
        .sum()
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Mul,
    Add,
}

impl Op {
    pub fn op(&self) -> Box<dyn Fn(u64, u64) -> u64> {
        match self {
            Op::Mul => Box::new(|a, b| a * b),
            Op::Add => Box::new(|a, b| a + b),
        }
    }
    pub fn new(x: &str) -> Self {
        match x {
            "*" => Self::Mul,
            "+" => Self::Add,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let ans = solve(
            r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ",
        );
        assert_eq!(ans, 3263827);
    }
}
