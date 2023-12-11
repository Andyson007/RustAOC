use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../example.txt");
    let mut index_val: HashMap<usize, u16> = HashMap::new();
    let mut tokens: Vec<Vec<Option<usize>>> = Vec::new();
    {
        let mut index = 1;
        let mut was_prev = false;
        let mut value: u16 = 0;
        for line in input.split('\n') {
            let mut tokenline: Vec<Option<usize>> = Vec::new();
            for number in line.chars() {
                if number.is_digit(10) {
                    value = value * 10 + number as u16 - 48;
                    tokenline.push(Some(index));
                    was_prev = true;
                } else {
                    tokenline.push(None);
                    if was_prev {
                        index_val.insert(index, value);
                        index += 1;
                        was_prev = false;
                    }
                    value = 0;
                }
            }
            tokenline.pop();
            tokens.push(tokenline.clone());
        }
        tokens.pop();
    }
    let mut sum: u32 = 0;
    for line in input.split('\n').enumerate() {
        for character in line.1.chars().enumerate() {
            if character.1 == '*' {
                let mut adj: HashSet<usize> = HashSet::new();
                if line.0 != 0 {
                    if character.0 != 0 {
                        if let Some(index) = tokens[line.0 - 1][character.0 - 1] {
                            adj.insert(index);
                        }
                    }
                    if let Some(index) = tokens[line.0 - 1][character.0] {
                        adj.insert(index);
                    }
                    if character.0 != line.1.len() - 1 {
                        if let Some(index) = tokens[line.0 - 1][character.0 + 1] {
                            adj.insert(index);
                        }
                    }
                }
                if line.0 != tokens.clone().len() - 1 {
                    if character.0 != 0 {
                        if let Some(index) = tokens[line.0 + 1][character.0 - 1] {
                            adj.insert(index);
                        }
                    }
                    if let Some(index) = tokens[line.0 + 1][character.0] {
                        adj.insert(index);
                    }
                    if character.0 != line.1.len() - 1 {
                        if let Some(index) = tokens[line.0 + 1][character.0 + 1] {
                            adj.insert(index);
                        }
                    }
                }
                if character.0 != 0 {
                    if let Some(index) = tokens[line.0][character.0 - 1] {
                        adj.insert(index);
                    }
                }
                if character.0 != line.1.len() - 1 {
                    if let Some(index) = tokens[line.0][character.0 + 1] {
                        adj.insert(index);
                    }
                }
                if adj.len() == 2 {
                    sum += adj
                        .iter()
                        .map(|val| *index_val.get(val).unwrap() as u32)
                        .fold(1u32, |sum, val| sum * val);
                }
            }
        }
    }
    print!("{sum}");
}
