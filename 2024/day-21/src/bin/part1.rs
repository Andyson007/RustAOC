use std::collections::HashMap;

use day_21::Pos;

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    let keypad = HashMap::from([
        ('A', Pos::new(2, 0)),
        ('0', Pos::new(1, 0)),
        ('1', Pos::new(0, 1)),
        ('2', Pos::new(1, 1)),
        ('3', Pos::new(2, 1)),
        ('4', Pos::new(0, 2)),
        ('5', Pos::new(1, 2)),
        ('6', Pos::new(2, 2)),
        ('7', Pos::new(0, 3)),
        ('8', Pos::new(1, 3)),
        ('9', Pos::new(2, 3)),
    ]);

    let arrow_keypad = HashMap::from([
        ('A', Pos::new(2, 1)),
        ('^', Pos::new(1, 1)),
        ('v', Pos::new(1, 0)),
        ('<', Pos::new(0, 0)),
        ('>', Pos::new(2, 0)),
    ]);

    raw.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .map(|x| {
            println!("\t{}", x.iter().collect::<String>());
            let diffs = calc_diffs(&keypad, &x);
            // println!("{diffs:?}");
            let once = diff_keypad(&diffs, Pos::new(0, 0));
            println!("{}", once.iter().collect::<String>());
            let once_diffs = calc_diffs(&arrow_keypad, &once);
            // println!("{once_diffs:?}");
            let twice = diff_keypad(&once_diffs, Pos::new(0, 1));
            println!("{}", twice.iter().collect::<String>());
            let twice_diffs = calc_diffs(&arrow_keypad, &twice);
            println!("{twice_diffs:?}");
            let three_times = diff_keypad(&twice_diffs, Pos::new(0, 1));
            println!("{}", three_times.iter().collect::<String>());
            let parsed_value = x.into_iter().collect::<String>()[0..3]
                .parse::<usize>()
                .unwrap();
            println!("{} * {parsed_value}", three_times.len());
            parsed_value * three_times.len()
        })
        .sum()
}

fn calc_diffs(keypad: &HashMap<char, Pos>, chars: &[char]) -> Vec<Pos> {
    let mut diffs = Vec::from([*keypad.get(&chars[0]).unwrap() - *keypad.get(&'A').unwrap()]);
    diffs.extend(
        chars
            .windows(2)
            .map(|x| *keypad.get(&x[1]).unwrap() - *keypad.get(&x[0]).unwrap()),
    );
    diffs
}

fn diff_keypad(diffs: &[Pos], invalid: Pos) -> Vec<char> {
    let mut curr = Pos::new(2, 1);
    let mut ret = Vec::new();
    for mut diff in diffs.iter().copied() {
        while diff != Pos::ZERO {
            for _ in 0..diff.y {
                if curr + Pos::NORTH != invalid {
                    diff -= Pos::NORTH;
                    curr += Pos::NORTH;
                    ret.push('^');
                }
            }
            for _ in 0..-diff.y {
                if curr + Pos::SOUTH != invalid {
                    diff -= Pos::SOUTH;
                    curr += Pos::SOUTH;
                    ret.push('v');
                }
            }
            for _ in 0..diff.x {
                if curr + Pos::EAST != invalid {
                    diff -= Pos::EAST;
                    curr += Pos::EAST;
                    ret.push('>');
                }
            }
            for _ in 0..-diff.x {
                if curr + Pos::WEST != invalid {
                    diff -= Pos::WEST;
                    curr += Pos::WEST;
                    ret.push('<');
                }
            }
        }
        ret.push('A');
    }
    ret
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        assert_eq!(solve(include_str!("../../example")), 126384);
    }
}
