fn main() {
    let input = include_str!("../../input.txt");
    let ans = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<i32>().unwrap())
                .rev()
                .collect::<Vec<i32>>()
        })
        .map(|x| deconstruct(x))
        .sum::<i32>();
    println!("{ans}");
}

fn deconstruct(sequence: Vec<i32>) -> i32 {
    if sequence.iter().all(|val| *val == 0) {
        return 0;
    }
    let new_vec = deconstruct(
        sequence
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<i32>>(),
    );
    new_vec + sequence.last().unwrap()
}
