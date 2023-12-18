use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .collect::<Vec<&str>>();
    let mut registers: HashMap<char, i32> = HashMap::from([('a', 0), ('b', 0), ('c', 1), ('d', 0)]);
    let mut index = 0;
    while input.len() > index {
        let line = input[index];
        let command = line.split_whitespace().collect::<Vec<&str>>();
        match command[0] {
            "cpy" => {
                let value = match command[1].parse::<i32>() {
                    Err(_) => *registers.get(&command[1].chars().nth(0).unwrap()).unwrap(),
                    Ok(x) => x,
                };
                *registers
                    .get_mut(&command[2].chars().nth(0).unwrap())
                    .unwrap() = value;
            }
            "inc" => {
                *registers
                    .get_mut(&command[1].chars().nth(0).unwrap())
                    .unwrap() += 1;
            }
            "dec" => {
                *registers
                    .get_mut(&command[1].chars().nth(0).unwrap())
                    .unwrap() -= 1;
            }
            "jnz" => {
                let value = match command[1].parse::<i32>() {
                    Err(_) => *registers.get(&command[1].chars().nth(0).unwrap()).unwrap(),
                    Ok(x) => x,
                };
                if value != 0 {
                    index = (index as i32 + command[2].parse::<i32>().unwrap()) as usize;
                    continue;
                }
            }
            _ => unreachable!(),
        }
        index += 1;
    }
    println!("{}", registers.get(&'a').unwrap());
}
