fn main() {
    let mut input = include_str!("../../input")
        .lines()
        .map(|line| {
            let split = line.split_once('-').unwrap();
            let split = (
                split.0.parse::<u32>().unwrap(),
                split.1.parse::<u32>().unwrap(),
            );
            if split.0 > split.1 {
                (split.1, split.0)
            } else {
                split
            }
        })
        .collect::<Vec<(u32, u32)>>();
    input.sort_by(|a, b| a.0.cmp(&b.0));
    let mut count = 0;
    let mut curr = 0;
    for elem in input {
        println!("\nelem: {elem:?}\ncurr: {curr}");
        if curr < elem.0 {
            count += elem.0 - curr - 1;
            // println!("{elem:?}");
            println!("{}", elem.0 - curr - 1);
        }
        curr = curr.max(elem.1);
    }
    count += u32::MAX - curr;
    println!("{count}");
}
