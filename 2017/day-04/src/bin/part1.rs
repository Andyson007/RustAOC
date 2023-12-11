fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| String::from(x))
                .collect::<Vec<String>>()
        })
        .filter(|line| {
            for i in 0..line.len() {
                for j in (i + 1)..line.len() {
                    if line[i] == line[j] {
                        return false;
                    }
                }
            }
            true
        })
        .count();
    println!("{ans}");
}
