fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .nth(0)
        .unwrap()
        .split(",");
    let (mut x, mut y) = (0isize, 0isize);
    for dir in input {
        match dir {
            "ne" => {
                x += 1;
                y += 1;
            }
            "se" => {
                x += 1;
                y -= 1;
            }
            "nw" => {
                x -= 1;
                y += 1;
            }
            "sw" => {
                x -= 1;
                y -= 1;
            }
            "n" => y += 2,
            "s" => y -= 2,
            _ => unreachable!(),
        }
    }
    println!("{}", std::cmp::max(x.abs(), (y.abs() + 1) / 2));
}
