use day_08::Image;

fn main() {
    let input = include_str!("../../input")
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let image = Image::<25, 6>::from(input);
    let ans = image.part1();
    println!("{ans}");
}
