use std::{collections::HashMap, io};

use day_13::{
    intcode::{IntCodeOutput, Intcode},
    Coord, Tile,
};

fn main() {
    let mut tiles = HashMap::<Coord, Tile>::new();
    let mut computer = Intcode::from_str(include_str!("../../input"));
    *computer.values.values.get_mut(&0).unwrap() = 2;

    let mut to_insert = Vec::with_capacity(3);
    let mut score = 0;

    let mut input = None;

    loop {
        match computer.compute(input.take().into_iter()) {
            IntCodeOutput::Blocking => {
                let ball_x = tiles.iter().find(|x| *x.1 == Tile::Ball).unwrap().0.x;
                let paddle_x = tiles
                    .iter()
                    .find(|x| *x.1 == Tile::HorizontalPaddle)
                    .unwrap()
                    .0
                    .x;
                let mut buf = String::new();
                input = match ball_x.cmp(&paddle_x) {
                    std::cmp::Ordering::Less => Some(-1),
                    std::cmp::Ordering::Equal => Some(0),
                    std::cmp::Ordering::Greater => Some(1),
                }
            }
            IntCodeOutput::Finished => break,
            IntCodeOutput::Output(x) => {
                to_insert.push(x);
                if let [a, b, c] = to_insert[..] {
                    if a == -1 && b == 0 {
                        score = c;
                    } else {
                        tiles.insert(Coord::new(a, b), Tile::from(c));
                        // if c == 3 {
                        //     panic!("{a} {b}");
                        // }
                    }
                    to_insert.clear();
                }
            }
        }
        // if !tiles.is_empty() {
        //     let min_x = tiles.iter().map(|x| x.0.x).min().unwrap();
        //     let max_x = tiles.iter().map(|x| x.0.x).max().unwrap();
        //     let min_y = tiles.iter().map(|x| x.0.y).min().unwrap();
        //     let max_y = tiles.iter().map(|x| x.0.y).max().unwrap();
        //     for i in min_y..=max_y {
        //         for j in min_x..=max_x {
        //             match tiles.get(&Coord::new(j, i)).unwrap_or(&Tile::Empty) {
        //                 Tile::Empty => print!(" "),
        //                 Tile::Wall => print!("█"),
        //                 Tile::Block => print!("▒"),
        //                 Tile::HorizontalPaddle => print!("-"),
        //                 Tile::Ball => print!("*"),
        //             }
        //         }
        //         println!()
        //     }
        // }
    }
    // println!("{}", tiles.iter().filter(|x| *x.1 == Tile::Block).count());
    println!("{score}");
}
