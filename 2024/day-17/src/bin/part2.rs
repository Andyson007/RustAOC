fn main() {
    let ans = solve([2, 4, 1, 5, 7, 5, 1, 6, 0, 3, 4, 2, 5, 5, 3, 0]);
    println!("{ans}");
}

fn solve<const N: usize>(program: [usize; N]) -> usize {
    let mut ans = 0;
    for len in 1..=N {
        for i in 0.. {
            let mut computer = Computer {
                a: (ans << 3) | i,
                b: 0,
                c: 0,
                program,
            };
            let solved = solve1(&mut computer);
            if solved.len() == len && program.ends_with(&solved) {
                ans = (ans << 3) | i;
                break;
            };
        }
    }
    ans
}

fn solve1<const N: usize>(input: &mut Computer<N>) -> Vec<usize> {
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
    use crate::solve;

    #[test]
    fn example() {
        assert_eq!(solve([0, 3, 5, 4, 3, 0]), 117440);
    }
}
