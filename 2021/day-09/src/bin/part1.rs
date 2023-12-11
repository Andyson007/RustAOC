fn main() {
    let input = include_str!("../../example.txt");

    input.lines().collect::<Vec<String>>().windows(3).collect::Vec<Vec<String>>();
}
