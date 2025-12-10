fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

pub fn solve(input: &str) -> i64 {
    let mut iter = input.lines().rev();
    let ops = iter
        .next()
        .unwrap()
        .split_whitespace()
        .map(Op::new)
        .collect::<Vec<_>>();
    iter.map(|x| {
        x.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    })
    .reduce(|a, b| {
        a.into_iter()
            .zip(b)
            .zip(ops.iter())
            .map(|((a, b), op)| op.op()(a, b))
            .collect::<Vec<i64>>()
    })
    .unwrap()
    .into_iter()
    .sum()
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Mul,
    Add,
}

impl Op {
    pub fn op(&self) -> Box<dyn Fn(i64, i64) -> i64> {
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
        assert_eq!(ans, 4277556);
    }
}
