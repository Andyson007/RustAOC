use std::cmp;

fn main() {
    let ans = include_str!("../../input.txt")
        .split("\r\n\r\n")
        .map(|mirrors| {
            let mirrors = mirrors
                .lines()
                .map(|line| line.chars().map(|x| x == '#').collect::<Vec<bool>>())
                .collect::<Vec<Vec<bool>>>();
            'outer: for i in 0..(mirrors.len() - 1) {
                let (a, b) = (mirrors[i].clone(), mirrors[i + 1].clone());
                for j in 0..mirrors[0].len() {
                    if a[j] != b[j] {
                        continue 'outer;
                    }
                }
                for k in 0..(cmp::min(i + 1, mirrors.len() - i - 1)) {
                    for j in 0..mirrors[0].len() {
                        if mirrors[i - k][j] != mirrors[i + k + 1][j] {
                            continue 'outer;
                        }
                    }
                }
                return (i + 1) * 100;
            }

            let mut rotated: Vec<Vec<bool>> = Vec::new();
            let mut tmp: Vec<bool> = Vec::new();
            tmp.resize(mirrors.len(), false);
            rotated.resize(mirrors[0].len(), tmp);
            for i in 0..mirrors[0].len() {
                for j in 0..mirrors.len() {
                    rotated[i][j] = mirrors[j][i];
                }
            }
            'outer: for i in 0..(rotated.len() - 1) {
                let (a, b) = (rotated[i].clone(), rotated[i + 1].clone());
                for j in 0..rotated[0].len() {
                    if a[j] != b[j] {
                        continue 'outer;
                    }
                }
                for k in 0..(cmp::min(i + 1, rotated.len() - i - 1)) {
                    for j in 0..rotated[0].len() {
                        if rotated[i - k][j] != rotated[i + k + 1][j] {
                            continue 'outer;
                        }
                    }
                }
                return i + 1;
            }
            unreachable!();
        })
        .map(|x| x as u32)
        .inspect(|x| println!("{x}"))
        .sum::<u32>();
    println!("{ans}")
}
