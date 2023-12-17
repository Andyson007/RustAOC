fn main() {
    let mut input = include_str!("../../input.txt").lines();
    let mut buses = input
        .nth(1)
        .unwrap()
        .split(",")
        .map(|x| {
            if x.chars().nth(0).unwrap() == 'x' {
                1
            } else {
                x.parse::<u128>().unwrap()
            }
        })
        .collect::<Vec<u128>>();
    let mut time: u128 = 0;
    let mut inc = 1;
    let mut offset = buses.len();
    loop {
        if buses.len() == 0 {
            break;
        }
        if (time + offset as u128) % buses.last().unwrap() == 0 {
            inc *= buses.last().unwrap();
            offset -= 1;
            buses.pop();
        } else {
            time += inc;
        }
    }
    time +=1;
    println!("{time}");
}
