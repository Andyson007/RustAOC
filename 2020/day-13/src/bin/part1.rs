fn main() {
    let mut input = include_str!("../../input.txt").lines();
    let time = input.next().unwrap().parse::<u32>().unwrap();
    let buses = input
        .next()
        .unwrap()
        .split(",")
        .filter(|x| x.chars().nth(0).unwrap() != 'x')
        .map(|x|x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let mut min = u32::MAX;
    let mut ans = 0;
    for bus in buses {
      println!("{bus}: {}", bus-time%bus);
      let wait = bus-time%bus;
      if wait <min {
        min = wait;
        ans = bus*wait;
      }
    }
    print!("{ans}");
}
