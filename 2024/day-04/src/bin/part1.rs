fn main() {
    let input = include_str!("../../input");
    let ans = solve(input);
    println!("{ans}");
}

fn solve(raw: &str) -> usize {
    let arr = raw
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut total: usize = 0;
    for x in 0..arr.len() as isize {
        for y in 0..arr[0].len() as isize {
            for dir_x in -1..=1 {
                for dir_y in -1..=1 {
                    if let Some(x) =  check(&arr, x, y, dir_x, dir_y) {
                        total += x;
                    };
                }
            }
        }
    }
    total
}

fn check(arr: &[Vec<char>], x: isize, y: isize, dir_x: isize, dir_y: isize) -> Option<usize> {
    let mut new_x = x;
    let mut new_y = y;
    for c in "XMAS".chars() {
        if *arr
            .get(usize::try_from(new_x).ok()?)?
            .get(usize::try_from(new_y).ok()?)?
            != c
        {
            None?
        };
        new_x += dir_x;
        new_y += dir_y;
    }
    Some(1)
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 18)
    }
}
