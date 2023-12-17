fn main() {
    let mut pos: (i32, i32) = (0, 0);
    let mut waypoint = (1, 10);
    for line in include_str!("../../input.txt").lines() {
        let command = line.chars().nth(0).unwrap();
        let value = line[1..line.len()].parse::<i32>().unwrap();
        println!("{pos:?} {waypoint:?}");
        match command {
            'N' => waypoint.0 += value,
            'E' => waypoint.1 += value,
            'S' => waypoint.0 -= value,
            'W' => waypoint.1 -= value,

            'F' => {
                pos.0 += value * waypoint.0;
                pos.1 += value * waypoint.1;
            }
            'L' => {
                waypoint = match value {
                    90 => (waypoint.1, -waypoint.0),
                    180 => (-waypoint.0, -waypoint.1),
                    270 => (-waypoint.1, waypoint.0),
                    _ => unreachable!(),
                }
            }
            'R' => {
                waypoint = match value {
                    270 => (waypoint.1, -waypoint.0),
                    180 => (-waypoint.0, -waypoint.1),
                    90 => (-waypoint.1, waypoint.0),
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{}", pos.0.abs() + pos.1.abs());
}
