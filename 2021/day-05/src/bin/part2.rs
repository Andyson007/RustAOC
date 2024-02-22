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
        .map(|points| Line {
            start: points.0,
            end: points.1,
        })
    {
        let incx = if end.x - start.x<0 {-1} else if end.x == start.x {0} else {1};
        let incy = if end.y - start.y<0 {-1} else if end.y == start.y {0} else {1};
        let mut curr = start;
        loop {
            if let Some(x) = points.get_mut(&curr) {
                *x+=1;
                if *x==2 {
                    count+=1;
                } 
            } else {
                points.insert(curr, 1);
            }
            if curr == end {
                break;
            }
            curr.x += incx;
            curr.y += incy;
        }
    }
    println!("{count}");
}
