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
            if let Some(x) = check(&arr, x, y) {
                total += x;
            };
        }
    }
    total
}

fn check(arr: &[Vec<char>], x: isize, y: isize) -> Option<usize> {
    if *arr
        .get(usize::try_from(x).ok()?)?
        .get(usize::try_from(y).ok()?)?
        != 'A'
    {
        return None;
    }

    let a = [
        *arr.get(usize::try_from(x + 1).ok()?)?
            .get(usize::try_from(y + 1).ok()?)?,
        *arr.get(usize::try_from(x - 1).ok()?)?
            .get(usize::try_from(y - 1).ok()?)?,
    ];
    let b = [
        *arr.get(usize::try_from(x - 1).ok()?)?
            .get(usize::try_from(y + 1).ok()?)?,
        *arr.get(usize::try_from(x + 1).ok()?)?
            .get(usize::try_from(y - 1).ok()?)?,
    ];
    if a.contains(&'M') && a.contains(&'S') && b.contains(&'M') && b.contains(&'S') {
        Some(1)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let input = include_str!("../../example");
        let ans = solve(input);
        assert_eq!(ans, 9)
    }
}
