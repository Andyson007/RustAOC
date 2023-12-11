fn main() {
    let numbers: [&str; 10] = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let ans = include_str!("../../input.txt")
        .lines()
        .map(|input| {
            let mut minpos = input.len();
            let mut min = 10;
            for number in numbers.iter().enumerate() {
                let pos = input.find(number.1);
                if let Some(pos) = pos {
                    if pos < minpos {
                        minpos = pos;
                        min = number.0;
                    }
                }
            }
            let pos = input.find(|c: char| c.is_digit(10));
            if let Some(pos) = pos {
                if pos < minpos {
                    min = (input.as_bytes()[pos] - 48) as usize;
                }
            }
            let mut maxpos = 0;
            let mut max = 0;
            for number in numbers.iter().enumerate() {
                let pos = input.rfind(number.1);
                if let Some(pos) = pos {
                    if pos >= maxpos {
                        maxpos = pos;
                        max = number.0;
                    }
                }
            }
            let pos = input.rfind(|c: char| c.is_digit(10));
            if let Some(pos) = pos {
                if pos >= maxpos {
                    max = (input.as_bytes()[pos] - 48) as usize;
                }
            }
            (10 * min + max) as u16
        })
        .sum::<u16>();
    println!("{ans}");
}
