use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

fn main() {
    let mut points: HashMap<Point, u32> = HashMap::new();
    let mut count = 0u32;
    for Line { start, end } in include_str!("../../input")
        .lines()
        .map(|line| {
            let split = line
                .split(" -> ")
                .map(|split| {
                    let mut split = split.split(',').map(|x| x.parse::<i32>().unwrap());
                    Point {
                        x: split.next().unwrap(),
                        y: split.next().unwrap(),
                    }
                })
                .collect::<Vec<Point>>();
            (split[0], split[1])
        })
        .filter(|(start, end)| start.x == end.x || start.y == end.y)
        .map(|points| Line {
            start: points.0,
            end: points.1,
        })
    {
        if start.y == end.y {
            let (min, max) = if start.x < end.x {
                (start.x, end.x)
            } else {
                (end.x, start.x)
            };
            for x in min..=max {
                let point = Point { x, y: start.y };
                if let Some(x) = points.get_mut(&point) {
                    *x += 1;
                    if *x == 2 {
                        count += 1;
                    }
                } else {
                    points.insert(point, 1);
                }
            }
        } else if start.x == end.x {
            let (min, max) = if start.y < end.y {
                (start.y, end.y)
            } else {
                (end.y, start.y)
            };
            for y in min..=max {
                let point = Point { x: start.x, y };
                if let Some(x) = points.get_mut(&point) {
                    *x += 1;
                    if *x == 2 {
                        count += 1;
                    }
                } else {
                    points.insert(point, 1);
                }
            }
        } else {
            unreachable!();
        }
    }
    println!("{count}");
}
