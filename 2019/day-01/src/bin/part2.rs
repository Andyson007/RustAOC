fn main() {
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .map(|mut val| {
            let mut ret = -val;
            while val >= 0 {
              ret+=val;
              val = val / 3 - 2;
            }
            // println!("{ret}");
            ret as u32
        })
        .sum::<u32>();
    println!("{ans}")
}
