fn main() {
    let input = include_str!("../../example.txt")
        .lines()
        .map(|line| {
            let line = line
                .split("~")
                .map(|x| {
                    x.split(",")
                        .map(|x| x.parse::<u8>().unwrap())
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<Vec<u8>>>();
            (
                (line[0][0], line[0][1], line[0][2]),
                (line[1][0], line[1][1], line[1][2]),
            )
        })
        .collect::<Vec<((u8, u8, u8), (u8, u8, u8))>>();
    for line in input {
      println!("{line:?}");
    }
}
