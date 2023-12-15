fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut totalsum = 0;
    for i in 0..input[0].len() {
        let mut posses: Vec<usize> = Vec::new();
        let mut start = 0;
        for j in 0..input.len() {
            if input[j][i] == 'O' {
              posses.push(start);
              start+=1;
            } else if input[j][i] == '#' {
              start = j + 1;
            }
        }
        let mut sum = 0;
        let dist = input[0].len();
        for pos in posses {
            sum += dist-pos;
        }
        totalsum+=sum;
    }
    println!("{totalsum}");
}
