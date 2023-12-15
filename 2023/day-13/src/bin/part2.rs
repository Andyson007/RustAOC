use std::cmp;

fn main() {
    let ans = include_str!("../../input.txt")
        .split("\r\n\r\n")
        .map(|mirrors| {
            let mirrors = mirrors
                .lines()
                .map(|line| line.chars().map(|x| x == '#').collect::<Vec<bool>>())
                .collect::<Vec<Vec<bool>>>();
            for i in 0..(mirrors.len() - 1) {
                let mut diffs = 0;
                for j in 0..mirrors[0].len() {
                    if mirrors[i][j] != mirrors[i + 1][j] {
                        diffs += 1;
                    }
                }
                for k in 1..(cmp::min(i + 1, mirrors.len() - i - 1)) {
                    for j in 0..mirrors[0].len() {
                        if mirrors[i - k][j] != mirrors[i + k + 1][j] {
                            diffs += 1;
                        }
                    }
                }
                if diffs == 1 {
                    return (i + 1) * 100;
                }
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
            for i in 0..(rotated.len() - 1) {
                let mut diffs = 0;
                for j in 0..rotated[0].len() {
                    if rotated[i][j] != rotated[i + 1][j] {
                        diffs += 1;
                    }
                }
                for k in 1..(cmp::min(i + 1, rotated.len() - i - 1)) {
                    for j in 0..rotated[0].len() {
                        if rotated[i - k][j] != rotated[i + k + 1][j] {
                            diffs += 1;
                        }
                    }
                }
                if diffs == 1 {
                    return i + 1;
                }
                // println!("{diffs}");
            }
            unreachable!();
        })
        .map(|x| x as u32)
        .sum::<u32>();
    println!("{ans}")
}
