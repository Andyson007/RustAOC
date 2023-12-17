fn main() {
    let mut pos: (i16, i16) = (0, 0);
    let mut dir = 0;
    for line in include_str!("../../input.txt").lines() {
        let command = line.chars().nth(0).unwrap();
        let value = line[1..line.len()].parse::<i16>().unwrap();
        println!("{line} ");
        match command {
            'N' => pos.0 += value,
            'E' => pos.1 += value,
            'S' => pos.0 -= value,
            'W' => pos.1 -= value,

            'R' => dir = (dir + value / 90) % 4,
            'L' => dir = (dir + 3*value / 90) % 4,
            'F' => match dir {
                0 => pos.1 += value,
                1 => pos.0 -= value,
                2 => pos.1 -= value,
                3 => pos.0 += value,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    println!("{}", pos.0.abs() + pos.1.abs());
}
