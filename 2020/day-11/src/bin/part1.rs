#[derive(Debug, Clone, Copy, PartialEq)]
enum Seat {
    Empty,
    Occupied,
}

#[derive(Debug, Clone, PartialEq)]
struct Seating {
    seating: Vec<Vec<Option<Seat>>>,
}

impl Seating {
    pub fn get(&self, x: i32, y: i32) -> Option<Option<Seat>> {
        if x < 0
            || y < 0
            || x as usize >= self.seating.len()
            || y as usize >= self.seating[x as usize].len()
        {
            None
        } else {
            Some(self.seating[x as usize][y as usize])
        }
    }
}

fn main() {
    let input = Seating {
        seating: include_str!("../../input.txt")
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| if c == 'L' { Some(Seat::Empty) } else { None })
                    .collect::<Vec<Option<Seat>>>()
            })
            .collect::<Vec<Vec<Option<Seat>>>>(),
    };
    let mut current = input.clone();
    loop {
        let mut new = Seating {
            seating: Vec::new(),
        };
        for (i, currentline) in current.seating.iter().enumerate() {
            let mut newline = Vec::new();
            for (j, seat) in currentline.iter().enumerate() {
                let mut count = 0;
                for x in -1..=1 {
                    for y in -1..=1 {
                        match current.get(i as i32 + x, j as i32 + y) {
                            Some(val) => match val {
                                Some(Seat::Occupied) => count += 1,
                                Some(Seat::Empty) => (),
                                None => (),
                            },
                            None => (),
                        }
                    }
                }
                newline.push(match seat {
                    None => None,
                    Some(x) => Some(match x {
                        Seat::Empty => {
                            if count == 0 {
                                Seat::Occupied
                            } else {
                                Seat::Empty
                            }
                        }
                        Seat::Occupied => {
                            if count >= 5 {
                                Seat::Empty
                            } else {
                                Seat::Occupied
                            }
                        }
                    }),
                    _ => unimplemented!(),
                })
            }
            new.seating.push(newline);
        }
        // for line in &new.seating {
        //     for seat in line {
        //         match seat {
        //             None => print!("."),
        //             Some(x) => match x {
        //                 Seat::Empty => print!("L"),
        //                 Seat::Occupied => print!("#"),
        //             },
        //         }
        //     }
        //     println!();
        // }
        // println!();
        if new.seating == current.seating {
            break;
        }
        current.seating = new.seating;
    }
    // for line in &current.seating {
    //     for seat in line {
    //         match seat {
    //             None => print!("."),
    //             Some(x) => match x {
    //                 Seat::Empty => print!("L"),
    //                 Seat::Occupied => print!("#"),
    //             },
    //         }
    //     }
    //     println!();
    // }
    let ans = current
        .seating
        .iter()
        .map(|line| {
            line.iter()
                .filter_map(|x| *x)
                .filter(|x| match x {
                    Seat::Occupied => true,
                    Seat::Empty => false,
                })
                .count()
        })
        .sum::<usize>();
    println!("{ans}");
}
