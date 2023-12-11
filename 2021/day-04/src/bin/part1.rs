fn main() {
    // Kinda wacky but the program represents all unmarked spaces with None
    // as they aren't important
    let input = include_str!("../../input.txt");
    let numbers = input
        .lines()
        .nth(0)
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap());
    let mut boards = input
        .split("\r\n\r\n")
        .skip(1)
        .map(|board| {
            board
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|x| Some(x.parse::<u8>().unwrap()))
                        .collect::<Vec<Option<u8>>>()
                })
                .collect::<Vec<Vec<Option<u8>>>>()
        })
        .collect::<Vec<Vec<Vec<Option<u8>>>>>();
    let mut ans = 1u32;
    'outer: for number in numbers {
        for board in boards.iter_mut() {
            for i in 0..5 {
                for j in 0..5 {
                    if board[i][j].is_none() {
                        continue;
                    }
                    if board[i][j].unwrap() == number {
                        board[i][j] = None;
                    }
                }
            }

            'next: for i in 0..5 {
                for j in 0..5 {
                    if board[i][j].is_some() {
                        continue 'next;
                    }
                }
                ans = score(board.clone()) * number as u32;
                break 'outer;
            }
            'next: for i in 0..5 {
                for j in 0..5 {
                    if board[j][i].is_some() {
                        continue 'next;
                    }
                }
                ans = score(board.clone()) * number as u32;
                break 'outer;
            }
        }
        // println!("{:#?}", boards);
    }
    println!("{ans}");
}

fn score(board: Vec<Vec<Option<u8>>>) -> u32 {
    let mut sum = 0;
    for a in board {
        for b in a {
            if let Some(x) = b {
                sum += x as u32;
            }
        }
    }
    sum
}
