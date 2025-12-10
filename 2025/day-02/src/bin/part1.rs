fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

fn solve(input: &str) -> usize {
    input
        .split(',')
        .map(|id| {
            let (start_str, end_str) = id.split_once('-').unwrap();
            eprintln!("{start_str} {end_str}");
            let (start, end): (usize, usize) =
                (start_str.parse().unwrap(), end_str.trim().parse().unwrap());
            layer(start, end)
        })
        .sum()
}

fn layer(start: usize, end: usize) -> usize {
    let mut sum = 0;
    let mut curr = start;
    while curr <= end {
        let digits = curr.ilog10() + 1;
        if digits % 2 == 1 {
            curr = 10usize.pow(digits);
            continue;
        }
        let half = 10usize.pow(digits / 2);
        let begin = curr / half;
        let full = begin * half + begin;
        if curr <= full {
            if full <= end {
                sum += full;
            }
            curr = full + 1;
        } else {
            curr = (begin + 1) * half + begin + 1
        }
    }

    sum
}
//
// #[cfg(test)]
// mod test {
//     use crate::{layer, solve};
//
//     #[test]
//     fn example() {
//         let ans = solve(
//             r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
//         );
//         assert_eq!(ans, 1227775554)
//     }
//
//     #[test]
//     fn layers() {
//         assert_eq!(layer(998, 1012), 1010);
//     }
// }
