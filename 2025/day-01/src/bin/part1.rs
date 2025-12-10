fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}")
}

fn solve(input: &str) -> usize{
    let mut curr = 50;
    let mut count = 0;
    for line in input.lines() {
        let dir = &line[0..=0];
        let steps = line[1..].parse::<i32>().unwrap();
        match dir {
            "R" => curr += steps,
            "L" => curr += 100-steps,
            _ => unreachable!()
        }
        curr %= 100;
        if curr == 0 {
            count += 1
        }
    }
    count

}

#[cfg(test)]
mod test {
    use crate::solve;

#[test]
    fn example() {
        let ans = solve(
            r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
",
        );
        assert_eq!(ans, 3);
    }
}
