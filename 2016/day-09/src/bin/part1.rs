fn main() {
    let input = include_str!("../../input.txt")
        .chars()
        .collect::<Vec<char>>();
    let mut count = 0;
    let mut index = 0;
    while index < input.len() {
      if input[index].is_whitespace() {
        index+=1;
        continue;
      }
        if input[index] == '(' {
            index += 1;
            let mut a = 0;
            while input[index] != 'x' {
                a = a * 10 + input[index].to_digit(10).unwrap();
                index += 1;
            }
            index += 1;
            let mut b = 0;
            while input[index] != ')' {
                b = b * 10 + input[index].to_digit(10).unwrap();
                index += 1;
            }
            index += a as usize + 1;
            count += a * b;
        } else {
            index += 1;
            count += 1;
        }
    }
    println!("{count}");
}
