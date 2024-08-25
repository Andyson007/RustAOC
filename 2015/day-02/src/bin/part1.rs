use itertools::Itertools;

fn main() {
    let ans: usize = include_str!("../../input")
        .lines()
        .map(|line| {
            let vals = line.splitn(3, 'x').map(|x| x.parse::<usize>().unwrap());
            let smallest = vals.clone().product::<usize>() / vals.clone().max().unwrap();
            let wrapping = 2 * vals.tuple_combinations().map(|(a, b)| a * b).sum::<usize>();
            smallest + wrapping
        })
        .sum();
    println!("{ans:?}");
}
