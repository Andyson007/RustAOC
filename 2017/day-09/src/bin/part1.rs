fn main() {
    let input = include_str!("../../input.txt")
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
        .map(|x| x.split('<').nth(0).unwrap())
        .fold(String::new(), |mut sum, curr| {
            sum.push_str(curr);
            sum
        });
    let mut level = 0;
    let mut score = 0;
    for c in input.chars() {
        match c {
            '{' => {
                level += 1;
                score += level;
            }
            '}' => {
                level -= 1;
            }
            _ => (),
        }
    }
    println!("{score}");
}
