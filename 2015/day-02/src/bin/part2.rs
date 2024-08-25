fn main() {
    let ans: usize = include_str!("../../input")
        .lines()
        .map(|line| {
            let vals = line.splitn(3, 'x').map(|x| x.parse::<usize>().unwrap());
            let max = vals.clone().max().unwrap();
            let sum = 2 * (vals.clone().sum::<usize>() - max);
            let volume: usize = vals.product();
            sum + volume
        })
        .sum();
    println!("{ans:?}");
}
