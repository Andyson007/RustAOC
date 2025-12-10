fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

pub fn solve(input: &str) -> usize {
    let points = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            Point {
                x: l.parse().unwrap(),
                y: r.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    let mut max = 0;
    for (pos, a) in points.iter().enumerate() {
        for b in &points[pos + 1..] {
            let size = area(a, b);
            if max < size {
                max = size
            }
        }
    }
    max
}

fn area(a: &Point, b: &Point) -> usize {
    (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
}

struct Point {
    x: usize,
    y: usize,
}

#[cfg(test)]
mod test {
    use crate::solve;

    #[test]
    fn example() {
        let ans = solve(
            r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        assert_eq!(ans, 50)
    }
}
