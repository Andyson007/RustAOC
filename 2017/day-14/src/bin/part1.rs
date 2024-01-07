fn main() {
    let mut count = 0;
    let input = "oundnydw";
    for i in 0..128{
      let mut to_eval = String::from(input);
      to_eval.push('-');
      to_eval.push_str(i.to_string().as_str());
        let input = to_eval
            .chars()
            .map(|c| c as u8)
            .chain([17, 31, 73, 47, 23])
            .collect::<Vec<u8>>();
        let mut current: u8 = 0;
        let mut skip: u8 = 0;
        let mut list = (0..=255).collect::<Vec<u8>>();
        for _ in 0..64 {
            for i in &input {
                for j in 0..i / 2 {
                    list.swap((current + j) as usize, (current + i - j - 1) as usize);
                }
                current += i + skip;
                skip += 1;
            }
        }
        count += list
            .chunks(16)
            .map(|x| x.iter().fold(0, |a, b| a ^ b))
            .map(|c| c.count_ones())
            .sum::<u32>();
    }
    println!("{count}");
}
