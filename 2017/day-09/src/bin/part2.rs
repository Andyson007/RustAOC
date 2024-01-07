fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .split_inclusive(|x| *x != '!')
        .filter_map(|x| {
            if x.len() % 2 == 1 {
                Some(x.last().unwrap())
            } else {
                None
            }
        })
        .collect::<String>()
        .split_inclusive(|x| x == '>')
        .rev()
        .skip(1)
        .map(|x| x.split_at(x.find('<').unwrap()).1.len() - 2)
        .sum::<usize>();
    println!("{ans}");
}
