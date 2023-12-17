fn main() {
    let mut input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c == 'R' || c == 'B')
                .fold(0, |sum, curr| sum * 2 + if curr { 1 } else { 0 })
        })
        .collect::<Vec<u16>>();
    input.sort();
    // print!("{input:#?}\n");
    for arr in input.windows(2) {
      if arr[0].abs_diff(arr[1]) >= 2 {
        println!("{}", arr[0]+1);
      }
    }
}
