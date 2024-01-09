fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| if c.is_whitespace() { None } else { Some(c) })
                .collect::<Vec<Option<char>>>()
        })
        .collect::<Vec<Vec<Option<char>>>>();
    let mut dir = (0, 1);
    let mut pos = (0, 0);
    for (i, c) in input[0].iter().enumerate() {
        if let Some(x) = c {
            if *x == '|' {
                pos = (i, 0);
            }
        }
    }
    let mut count = 0;
    loop {
        if let Some(x) = input[pos.1][pos.0] {
            if x != '+' {
                // if x.is_alphabetic() {
                //     print!("{x}");
                // }
            } else {
                if dir.0 == 0 {
                    if input[pos.1].len() != pos.0 + 1 {
                        if let Some(_) = input[pos.1][pos.0 + 1] {
                            dir = (1, 0);
                        }
                    }
                    if pos.0 != 0 {
                        if let Some(_) = input[pos.1][pos.0 - 1] {
                            dir = (-1, 0);
                        }
                    }
                } else {
                    if pos.1 + 1 != input.len() {
                        if let Some(_) = input[pos.1 + 1][pos.0] {
                            dir = (0, 1);
                        }
                    }
                    if pos.1 != 0 {
                        if let Some(_) = input[pos.1 - 1][pos.0] {
                            dir = (0, -1);
                        }
                    }
                }
            }
            pos = (
                (pos.0 as i16 + dir.0) as usize,
                (pos.1 as i16 + dir.1) as usize,
            );
        } else {
            break;
        }
        count += 1;
    }
    println!("{count}");
}
