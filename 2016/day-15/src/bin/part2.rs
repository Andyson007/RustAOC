fn main() {
    let input = include_str!("../../input")
        .lines()
        .map(|line| {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            (
                split[3].parse::<usize>().unwrap(),
                split[11].trim_end_matches('.').parse::<usize>().unwrap(),
            )
        })
        .chain([(11, 0)]);
    let mut time = 0;
    let mut increment = 1;
    for (i, (cycle, startpos)) in input.enumerate() {
        loop {
            if (startpos + i + time + 1) % cycle == 0 {
                increment *= cycle;
                break;
            }
            time += increment;
        }
    }
    println!("{time}");
}
