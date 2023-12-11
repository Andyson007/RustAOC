fn main() {
    let input = include_str!("../../input.txt");
    let mut symbols: Vec<Vec<bool>> = Vec::new();
    for line in input.split("\n").enumerate() {
        let mut current: Vec<bool> = Vec::new();
        for character in line.1.chars() {
            current.push(!character.is_digit(10) && character != '.');
        }
        current.pop();
        symbols.push(current.clone());
    }
    let mut possible: Vec<Vec<bool>> = Vec::new();
    for line in symbols.iter().enumerate() {
        let mut current: Vec<bool> = Vec::new();
        for pos in line.1.iter().enumerate() {
            let mut num: bool = false;
            if line.0 != 0 {
                if pos.0 != 0 {
                    num |= symbols[line.0 - 1][pos.0 - 1];
                }
                num |= symbols[line.0 - 1][pos.0];
                if pos.0 != line.1.len() - 1 {
                    num |= symbols[line.0 - 1][pos.0 + 1];
                }
            }
            if line.0 != symbols.len() - 2 {
                if pos.0 != 0 {
                    num |= symbols[line.0 + 1][pos.0 - 1];
                }
                num |= symbols[line.0 + 1][pos.0];
                if pos.0 != line.1.len() - 1 {
                    num |= symbols[line.0 + 1][pos.0 + 1];
                }
            }
            if pos.0 != 0 {
                num |= symbols[line.0][pos.0 - 1];
            }
            if pos.0 != line.1.len() - 1 {
                num |= symbols[line.0][pos.0 + 1];
            }
            current.push(num);
        }
        possible.push(current);
    }
    let mut sum: u32 = 0;
    for line in input.split("\n").enumerate() {
        let mut val: u32 = 0;
        let mut ignore = true;
        for pos in line.1.chars().enumerate() {
            if pos.1.is_digit(10) {
                if possible[line.0][pos.0] {
                    ignore = false;
                }
                val = 10 * val + pos.1 as u32 - 48;
            } else {
                if !ignore {
                    sum += val;
                }
                val = 0;
                ignore = true;
            }
        }
    }
    println!("{sum}")
}
