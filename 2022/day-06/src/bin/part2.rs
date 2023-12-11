fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .position(|window| {
            for i in 0..window.len() {
                for j in (i + 1)..window.len() {
                    if window[i] == window[j] {
                        return false;
                    }
                }
            }
            true
        });
    println!("{}", ans.unwrap() + 14);
}
