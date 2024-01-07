fn main() {
    let input = include_str!("../../input.txt")
// "30373
// 25512
// 65332
// 33549
// 35390
// "
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    let mut max = 0;
    for i in 0..input.len() {
        for j in 0..input[i].len() {
    // let i = 1;
    // let j = 2;
    // {
    //     {
            let mut score = 1;
            let height = input[i][j];
            {
                let mut count = 0;
                let mut treeheight = 0;
                for xloc in (0..i).rev() {
                    if input[xloc][j] >= treeheight {
                        treeheight = input[xloc][j];
                        count += 1;
                        if treeheight >= height {
                            break;
                        }
                    }
                }
                score *= count;
            }

            {
                let mut count = 0;
                let mut treeheight = 0;
                for xloc in i + 1..input.len() {
                    if input[xloc][j] >= treeheight {
                        treeheight = input[xloc][j];
                        count += 1;
                        if treeheight >= height {
                            break;
                        }
                    }
                }
                score *= count;
            }

            {
                let mut count = 0;
                let mut treeheight = 0;
                for yloc in (0..j).rev() {
                    if input[i][yloc] >= treeheight {
                        treeheight = input[i][yloc];
                        count += 1;
                        if treeheight >= height {
                            break;
                        }
                    }
                }
                score *= count;
            }

            {
                let mut count = 0;
                let mut treeheight = 0;
                for yloc in j + 1..input[i].len() {
                    if input[i][yloc] >= treeheight {
                        treeheight = input[i][yloc];
                        count += 1;
                        if treeheight >= height {
                            break;
                        }
                    }
                }
                score *= count;
            }

            max = std::cmp::max(score, max);
        }
    }
    println!("{max}");
}
