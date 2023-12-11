fn main() {
    let input = include_str!("../../example.txt")
        .split(",")
        .map(|val| val.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
}
