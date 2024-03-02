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
    let mut childvec = Vec::new();
    for _ in 0..children {
        let child = solve(index, input);
        index = child.0;
        childvec.push(child);
    }
    let mut sum = 0;
    if children == 0 {
        for _ in 0..metadata {
            sum += input[index];
            index += 1;
        }
    } else {
        for _ in 0..metadata {
            sum += childvec.get(input[index] as usize - 1).unwrap_or(&(0, 0)).1;
            index += 1;
        }
    }
    (index, sum)
}
