fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

fn solve(input: &str) -> usize {
    input
        .split(',')
        .map(|id| {
            let (start_str, end_str) = id.split_once('-').unwrap();
            let (start, end): (usize, usize) =
                (start_str.parse().unwrap(), end_str.trim().parse().unwrap());
            (start..=end)
                .filter(|curr: &usize| is_invalid(*curr))
                .sum::<usize>()
        })
        .sum()
}

fn is_invalid(curr: usize) -> bool {
    (1..=curr.ilog10()).any(|len| is_invalid_with_len(curr, len))
}

fn is_invalid_with_len(curr: usize, repeat_len: u32) -> bool {
    let digits = curr.ilog10() + 1;
    if !digits.is_multiple_of(repeat_len) {
        return false;
    }
    let split = curr % 10usize.pow(repeat_len);

    let mut num = 0;
    for _ in 0..digits / repeat_len {
        num *= 10usize.pow(repeat_len);
        num += split;
    }
    num == curr
}

#[cfg(test)]
mod test {
    use crate::{is_invalid, solve};

    #[test]
    fn example() {
        let ans = solve(
            r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        assert_eq!(ans, 4174379265)
    }

    #[test]
    fn invalids() {
        assert!(is_invalid(999));
        assert!(is_invalid(1010));
        assert!(is_invalid(11));
        assert!(is_invalid(22));
        assert!(!is_invalid(1188511880));
        assert!(!is_invalid(12));
    }
}

bare kilder:
- [Kvinner jaktet også](https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0287101)
- [intersex sex determination](https://europepmc.org/article/med/9651461)
- [2% av fødsler er ikke idelle menn/kvinner](https://europepmc.org/article/med/11534012)
- [two year followup on HRT](https://www.nejm.org/doi/full/10.1056/NEJMoa2206297#t2)
- The boy who was raised girl (David reimer)
