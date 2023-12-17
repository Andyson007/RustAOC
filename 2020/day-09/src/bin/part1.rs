fn main() {
    let size = 25;
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .windows(size + 1)
        .find(|arr| {
            let goal = arr[size];
            for i in 0..size {
                for j in (i + 1)..size {
                    if arr[i] + arr[j] == goal {
                        return false;
                    }
                }
            }
            true
        }).map(|arr|arr[size])
        .unwrap();
    println!("{ans}");
}
