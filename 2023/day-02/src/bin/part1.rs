#[derive(Debug, Clone)]
struct Cube {
    red: u8,
    green: u8,
    blue: u8,
}
fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if line[line.find(":").unwrap() + 2..].split("; ").all(|round| {
                let total = round.split(", ").fold(
                    Cube {
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                    |sum, condition| {
                        let mut ret = sum.clone();
                        let amount = condition
                            .split(" ")
                            .take(1)
                            .collect::<String>()
                            .parse::<u8>()
                            .unwrap();
                        match condition
                            .split(" ")
                            .skip(1)
                            .collect::<String>()
                            .as_str()
                            .trim()
                        {
                            "red" => ret.red += amount,
                            "green" => ret.green += amount,
                            "blue" => ret.blue += amount,
                            _ => println!("bad!"),
                        }
                        ret
                    },
                );
                total.red <= 12 && total.green <= 13 && total.blue <= 14
            }) {
                i + 1
            } else {
                0
            }
        })
        .sum::<usize>();

    println!("{ans}");
}
