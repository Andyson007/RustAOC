fn main() {
    let input = include_str!("../../input.txt");
    let crabs = input
        .split(",")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut min = !0u32;
    for position in (*crabs.iter().min().unwrap())..(*crabs.iter().max().unwrap()) {
        let mut fuel: u32 = 0;
        for crab in crabs.clone() {
            let dist  = (crab - position).abs() as u32;
            fuel += dist*(dist+1)/2;
        }
        min = if fuel < min {fuel} else {min};
    }
    println!("{min}");
}
