use std::collections::HashMap;

fn main() {
    let mut registers: HashMap<&str, isize> = HashMap::new();
    let mut max = 0;
    for line in include_str!("../../input.txt").lines() {
        let split = line.split_whitespace().collect::<Vec<&str>>();
        let lval = *registers.get(split[4]).unwrap_or(&0);
        let rval = split[6].parse::<isize>().unwrap();
        let modifier = split[2].parse::<isize>().unwrap();
        let modifier = match split[1] {
            "inc" => modifier,
            "dec" => -modifier,
            _ => unreachable!(),
        };

        if match split[5] {
            ">" => lval > rval,
            ">=" => lval >= rval,
            "<" => lval < rval,
            "<=" => lval <= rval,
            "==" => lval == rval,
            "!=" => lval != rval,
            _ => unreachable!(),
        } {
            if let Some(x) = registers.get_mut(split[0]) {
                *x += modifier;
                max = std::cmp::max(max, *x);
            } else {
                registers.insert(split[0], modifier);
                max = std::cmp::max(max, modifier);
            }
        }
    }
    println!("{max}");
}
