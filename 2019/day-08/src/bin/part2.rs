use day_08::Image;

fn main() {
    println!("\x1b[93mError\x1b[0m");
    let input = include_str!("../../input")
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let image = Image::<25, 6>::from(input);
    let ans = image.part2();
    for col in 0..6 {
        for row in 0..25 {
            match ans[row][col] {
                Some(true) => print!("\u{2588}\u{2588}"),
                Some(false) => print!("  "),
                None => print!("**"),
            }
        }
        println!();
    }
}
