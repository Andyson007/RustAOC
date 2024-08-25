fn main() {
    let ans = include_str!("../../input")
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| match x {
            '(' => 1,
            ')' => -1,
            _ => 0,
        })
        .scan(0, |state, curr| {
            *state += curr;
            Some(*state)
        })
        .take_while(|x| *x >= 0)
        .count()
        + 1;
    println!("{ans}")
}
