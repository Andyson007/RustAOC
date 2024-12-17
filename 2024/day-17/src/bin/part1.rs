fn main() {
    let mut input = Computer {
        a: 44348299,
        b: 0,
        c: 0,
        program: [2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 2, 5, 5, 3, 0],
    };
    let ans = solve(&mut input);
    let mut to_print = String::new();
    for val in ans {
        to_print.push_str(&val.to_string());
        to_print.push(',');
    }
    to_print.pop();
    println!("{to_print}");
}

fn solve<const N: usize>(input: &mut Computer<N>) -> Vec<usize> {
    let mut position = 0;
    let mut ans = Vec::new();
    while position < N {
        let combo = {
            let raw = input.program[position + 1];
            match raw {
                x @ 0..=3 => Some(x),
                4 => Some(input.a),
                5 => Some(input.b),
                6 => Some(input.c),
                _ => None,
            }
        };
        match input.program[position] {
            0 => input.a /= 1 << combo.unwrap(),
            1 => input.b ^= input.program[position + 1],
            2 => input.b = combo.unwrap() % 8,
            3 => {
                if input.a != 0 {
                    position = input.program[position + 1];
                    continue;
                }
            }
            4 => input.b ^= input.c,
            5 => ans.push(combo.unwrap() % 8),
            6 => input.b = input.a / (1 << combo.unwrap()),
            7 => input.c = input.a / (1 << combo.unwrap()),
            _ => unreachable!(),
        }

        position += 2;
    }
    ans
}

struct Computer<const N: usize> {
    a: usize,
    b: usize,
    c: usize,
    program: [usize; N],
}

#[cfg(test)]
mod test {
    use crate::{solve, Computer};

    #[test]
    fn example() {
        assert_eq!(
            solve(&mut Computer {
                a: 729,
                b: 0,
                c: 0,
                program: [0, 1, 5, 4, 3, 0],
            }),
            [4, 6, 3, 5, 6, 3, 5, 2, 1, 0,]
        )
    }

    #[test]
    fn a() {
        let mut computer = Computer {
            a: 0,
            b: 0,
            c: 9,
            program: [2, 6],
        };
        assert_eq!(solve(&mut computer), []);
        assert_eq!(computer.b, 1);
    }

    #[test]
    fn b() {
        assert_eq!(
            solve(&mut Computer {
                a: 10,
                b: 0,
                c: 0,
                program: [5, 0, 5, 1, 5, 4]
            }),
            [0, 1, 2]
        )
    }

    #[test]
    fn c() {
        assert_eq!(
            solve(&mut Computer {
                a: 2024,
                b: 0,
                c: 0,
                program: [0, 1, 5, 4, 3, 0]
            }),
            [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]
        )
    }

    #[test]
    fn d() {
        let mut computer = Computer {
            a: 0,
            b: 29,
            c: 0,
            program: [1, 7],
        };
        assert_eq!(solve(&mut computer), []);
        assert_eq!(computer.b, 26);
    }

    #[test]
    fn e() {
        let mut computer = Computer {
            a: 0,
            b: 2024,
            c: 43690,
            program: [4, 0],
        };
        assert_eq!(solve(&mut computer), []);
        assert_eq!(computer.b, 44354);
    }
}
