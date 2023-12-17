fn main() {
    let mut lines = include_str!("../../input.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    lines.sort();
    lines.insert(0, 0);
    lines.push(lines.last().unwrap()+3);
    let diffs = lines
        .windows(2)
        .map(|arr| arr[1] - arr[0])
        .collect::<Vec<u64>>();
    let mut ones: Vec<u64> = Vec::new();
    let mut count = 0;
    for d in diffs.clone() {
        if d == 3 {
            if count != 0 {
                ones.push(count);
            }
            count = 0;
        } else if d == 1 {
            count += 1;
        } else {
            unreachable!();
        }
    }
    println!("{lines:?}");
    println!("{diffs:?}");
    println!("{ones:?}");
    let ans = ones
        .iter()
        .map(|x| match x {
            1 => 1,
            2 => 2,
            3 => 4,
            4 => 7,
            _ => 0,
        })
        .product::<u64>();
        println!("{ans}");
}
