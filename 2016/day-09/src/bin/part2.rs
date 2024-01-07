fn main() {
    let input = include_str!("../../input.txt")
        .chars()
        .filter(|x| !x.is_whitespace())
        .collect::<Vec<char>>();
    let ans = solve(&input, 0, input.len());
    println!("{ans}");
}

fn solve(compressed: &Vec<char>, begin: usize, end: usize) -> usize {
    // for i in begin..end {
    //     print!("{}", compressed[i]);
    // }
    // println!();
    let mut index = begin;
    let mut count = 0;
    while index < end {
        if compressed[index] == '(' {
            index += 1;
            let mut a = 0;
            while compressed[index] != 'x' {
                a = a * 10 + compressed[index].to_digit(10).unwrap();
                index += 1;
            }
            index += 1;
            let mut b = 0;
            while compressed[index] != ')' {
                b = b * 10 + compressed[index].to_digit(10).unwrap();
                index += 1;
            }
            count += solve(&compressed, index + 1, index + a as usize + 1) * b as usize;
            index += a as usize + 1;
        } else {
            index += 1;
            count += 1;
        }
    }
    count
}
