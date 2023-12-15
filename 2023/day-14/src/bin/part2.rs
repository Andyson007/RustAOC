use std::collections::HashMap;

fn main() {
    let mut input = include_str!("../../input.txt")
        .lines()
        .map(|line| line.chars().rev().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut copyinput = input.clone();
    let mut dupecheck: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    let mut cycle;
    let mut i = 0;
    loop {
        for _ in 0..4 {
            let mut newinput: Vec<Vec<char>> = Vec::new();
            let mut tmp: Vec<char> = Vec::new();
            tmp.resize(copyinput.len(), '.');
            newinput.resize(copyinput.len(), tmp.clone());
            let len = (copyinput.len(), tmp.len());
            for i in 0..copyinput[0].len() {
                let mut start = 0;
                for j in 0..copyinput.len() {
                    if copyinput[j][i] == 'O' {
                        newinput[len.0 - i - 1][start] = 'O';
                        start += 1;
                    } else if copyinput[j][i] == '#' {
                        start = j + 1;
                        newinput[len.0 - i - 1][j] = '#';
                    }
                }
            }
            copyinput = newinput;
        }
        if let Some(x) = dupecheck.get(&copyinput) {
            cycle = (i, *x);
            break;
        } else {
            dupecheck.insert(copyinput.clone(), i);
        }
        i += 1;
    }
    println!("{cycle:?}");
    let mut totalsum = 0;
    // let mut input = copyinput.clone();
    let iters = 1000000000 % (cycle.0 - cycle.1);
    for i in 0..(iters + cycle.0 - 2) {
        for _ in 0..4 {
            let mut newinput: Vec<Vec<char>> = Vec::new();
            let mut tmp: Vec<char> = Vec::new();
            tmp.resize(input.len(), '.');
            newinput.resize(input.len(), tmp.clone());
            let len = (input.len(), tmp.len());
            for i in 0..input[0].len() {
                let mut start = 0;
                for j in 0..input.len() {
                    if input[j][i] == 'O' {
                        newinput[len.0 - i - 1][start] = 'O';
                        start += 1;
                    } else if input[j][i] == '#' {
                        start = j + 1;
                        newinput[len.0 - i - 1][j] = '#';
                    }
                }
            }
            input = newinput;
        }
        if i % 10000 == 0 {
            // println!("{i}");
        }
    }
    for i in 0..input[0].len() {
        let mut sum = 0;
        let dist = input.len();
        for j in 0..input.len() {
            if input[j][i] == 'O' {
                sum += dist - j;
            }
        }
        totalsum += sum;
    }
    println!("{totalsum}");
}
