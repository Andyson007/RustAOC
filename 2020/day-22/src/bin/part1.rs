fn main() {
    let input = include_str!("../../input.txt")
        .split("\r\n\r\n")
        .map(|lines| {
            lines
                .lines()
                .skip(1)
                .map(|line| line.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let mut players = (input[0].clone(), input[1].clone());
    // for i in input.clone() {
    //   println!("{i:?}");
    // }
    let vec = loop {
        if players.0.len() == 0 {
            break players.1.clone();
        } else if players.1.len() == 0 {
            break players.0.clone();
        }
        let cards = (players.0[0], players.1[0]);
        players.0.remove(0);
        players.1.remove(0);
        if cards.0 < cards.1 {
            players.1.push(cards.1);
            players.1.push(cards.0);
        } else {
            players.0.push(cards.0);
            players.0.push(cards.1);
        }
    };
    let ans = vec
        .iter()
        .rev()
        .enumerate()
        .map(|(i, val)| *val as u32 * (i as u32 + 1))
        .sum::<u32>();
    println!("{ans}");
}
