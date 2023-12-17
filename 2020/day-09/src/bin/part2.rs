use std::cmp;

fn main() {
    let size = 25;
    let mut input = include_str!("../../input.txt")
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let ans = input
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
        })
        .map(|arr| arr[size])
        .unwrap();
    for i in 0..input.len() {
        'inner: for j in i..input.len() {
            let mut sum = 0;
            for k in i..=j {
                sum += input[k];
                if sum > ans {
                    continue 'inner;
                } else if sum == ans {
                    let mut min = u64::MAX;
                    let mut max = 0;
                    for k in i..=j {
                        min = cmp::min(input[k], min);
                        max = cmp::max(input[k], max);
                    }
                    println!("{min} {max}\n{}", min+max);
                    return;
                }
            }
        }
    }
}
