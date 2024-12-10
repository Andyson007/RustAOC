use std::iter;

fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    let first_line = raw
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .enumerate()
        .flat_map(|(id, elem)| {
            if id & 1 == 0 {
                iter::repeat_n(Some(id / 2), elem)
            } else {
                iter::repeat_n(None, elem)
            }
        })
        .collect::<Vec<_>>();
    let mut forwards = first_line.iter();
    let mut backwards = first_line
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(pos, elem)| elem.as_ref().map(|x| (pos, x)));
    let mut sum = 0;
    let mut backwards_pos = first_line.len() - 1;
    for pos in 0.. {
        if pos >= backwards_pos {
            break;
        }
        if let Some(x) = forwards.next().unwrap() {
            sum += pos * x;
        } else {
            let (new_backwards, x) = backwards.next().unwrap();
            backwards_pos = new_backwards;
            sum += pos * x;
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 1928);
    }
}
