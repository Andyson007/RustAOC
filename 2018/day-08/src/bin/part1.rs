fn main() {
    let input = include_str!("../../input")
        .lines()
        .next()
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let (_, ans) = solve(0, &input);
    println!("{ans}");
}

fn solve(mut index: usize, input: &Vec<u32>) -> (usize, u32) {
    let children = input[index];
    let metadata = input[index + 1];
    index += 2;
    let mut sum = 0;
    for _ in 0..children {
        let child = solve(index, input);
        index = child.0;
        sum += child.1;
    }
    for _ in 0..metadata {
        sum += input[index];
        index += 1;
    }
    (index, sum)
}
