fn main() {
    let ans = include_str!("../../input.txt")
      .lines()
        .map(|input| {
            let first = input.find(|x: char| x.is_digit(10));
            let last = input.rfind(|x: char| x.is_digit(10));
            ((input.as_bytes()[first.unwrap()] - 48) * 10 + input.as_bytes()[last.unwrap()] - 48)
                as u16
        })
        .sum::<u16>();
    println!("{ans}");
}
