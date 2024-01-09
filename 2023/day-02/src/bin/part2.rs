use std::cmp;

#[derive(Debug, Clone)]
struct Cube {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .enumerate()
        .map(|line| {
            let pos = line.1.find(":");
            if pos.is_none() {
                return Cube {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
            }
            line.1[pos.unwrap() + 2..].split("; ").fold(
                Cube {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |sum: Cube, round| {
                    let mut total = round.split(", ").fold(
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
                                .parse::<u32>()
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
                    total.red = cmp::max(total.red, sum.red);
                    total.green = cmp::max(total.green, sum.green);
                    total.blue = cmp::max(total.blue, sum.blue);
                    total
                },
            )
        })
        .fold(0u32, |sum: u32, curr: Cube| {
            sum + (curr.red * curr.green * curr.blue) as u32
        });
    println!("{ans}");
}
