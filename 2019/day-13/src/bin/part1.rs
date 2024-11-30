use std::collections::HashMap;

use day_13::{
    intcode::{IntCodeOutput, Intcode},
    Coord, Tile,
};

fn main() {
    let mut tiles = HashMap::<Coord, Tile>::new();
    let mut computer = Intcode::from_str(include_str!("../../input"));

    let mut to_insert = Vec::with_capacity(3);

    loop {
        match computer.compute(std::iter::empty()) {
            IntCodeOutput::Blocking => panic!(),
            IntCodeOutput::Finished => break,
            IntCodeOutput::Output(x) => {
                to_insert.push(x);
                if let [a, b, c] = to_insert[..] {
                    tiles.insert(Coord::new(a, b), Tile::from(c));
                    to_insert.clear();
                }
            }
        }
    }
    println!("{}", tiles.iter().filter(|x| *x.1 == Tile::Block).count());
}
