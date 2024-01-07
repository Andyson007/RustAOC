fn main() {
    let first = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            let mut val = 0;
            for c in line.chars() {
                val *= 5;
                match c {
                    '2' => val += 2,
                    '1' => val += 1,
                    '0' => (),
                    '-' => val -= 1,
                    '=' => val -= 2,
                    _ => unreachable!(),
                }
            }
            val
        })
        .sum::<u64>();
    // println!("{first}");
    let mut five = 0;
    while five < first {
        five = five * 5 + 2;
    }
    let mut temp = five + first;
    let mut converted = 0;
    while temp > 0 {
        converted = converted * 5 + temp % 5;
        temp /= 5;
    }
    println!("{converted}");
    let mut tmp = String::new();
    while converted > 0 {
        tmp.push(match converted % 5 {
            4 => '2',
            3 => '1',
            2 => '0',
            1 => '-',
            0 => '=',
            _ => unreachable!(),
        });
        converted /= 5;
    }
    println!("{tmp}");
}
