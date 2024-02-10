#[derive(Debug)]
enum Command {
    Rect { x: u32, y: u32 },
    Row { row: u32, dist: u32 },
    Column { column: u32, dist: u32 },
}

fn main() {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|x| {
            let split = x.split_whitespace().collect::<Vec<&str>>();
            if split[0] == "rect" {
                let vals = split[1]
                    .split("x")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                Command::Rect {
                    x: vals[0],
                    y: vals[1],
                }
            } else {
                let first = split[2].split("=").last().unwrap().parse::<u32>().unwrap();
                let dist = split[4].parse::<u32>().unwrap();
                if split[1] == "row" {
                    Command::Row { row: first, dist }
                } else {
                    Command::Column {
                        column: first,
                        dist,
                    }
                }
            }
        })
        .collect::<Vec<Command>>();
    let mut pixels = [[false; 6]; 50];
    for line in input {
        match line {
            Command::Rect { x, y } => {
                for i in 0..x {
                    for j in 0..y {
                        pixels[i as usize][j as usize] = true;
                    }
                }
            }
            Command::Column { column, dist } => {
                let mut columndata = [false; 6];
                for (index, val) in pixels[column as usize].iter().enumerate() {
                    columndata[(index + dist as usize) % 6] = *val;
                }
                pixels[column as usize] = columndata;
            }
            Command::Row { row, dist } => {
                let mut rowdata = [false; 50];
                for index in 0..50 {
                    rowdata[(index + dist as usize) % 50] = pixels[index][row as usize];
                }
                for index in 0..50 {
                    pixels[index][row as usize] = rowdata[index];
                }
            }
        }
    }
    let ans = pixels
        .iter()
        .map(|x| x.iter().filter(|x| **x).count())
        .sum::<usize>();
    println!("{ans}");
}
