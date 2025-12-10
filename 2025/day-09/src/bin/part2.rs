use std::{
    cmp::{max, min},
    iter,
};

fn main() {
    let ans = solve(include_str!("../../input"));
    println!("{ans}");
}

pub fn solve(input: &str) -> usize {
    let points = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(',').unwrap();
            Point {
                x: l.parse().unwrap(),
                y: r.parse().unwrap(),
            }
        })
        .collect::<Vec<_>>();
    let border: Vec<_> = points
        .windows(2)
        .chain(iter::once(
            [*points.first().unwrap(), *points.last().unwrap()].as_slice(),
        ))
        .map(|arr| Line::new(arr[0], arr[1]))
        .collect();
    let mut max = 0;
    for (pos, a) in points.iter().enumerate() {
        for b in &points[pos + 1..] {
            let rect = Rect::new(*a, *b);
            if !rect.is_invalid(&border) {
                let size = rect.area();
                if max < size {
                    println!("{size}: {rect:?}");
                    max = size;
                }
            }
        }
    }
    max
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    top_left: Point,
    bottom_right: Point,
}

impl Rect {
    pub fn new(a: Point, b: Point) -> Self {
        Self {
            top_left: Point {
                x: min(a.x, b.x),
                y: min(a.y, b.y),
            },
            bottom_right: Point {
                x: max(a.x, b.x),
                y: max(a.y, b.y),
            },
        }
    }

    pub fn area(&self) -> usize {
        (self.top_left.x.abs_diff(self.bottom_right.x) + 1)
            * (self.top_left.y.abs_diff(self.bottom_right.y) + 1)
    }

    pub fn is_invalid(&self, border: &[Line]) -> bool {
        for line in border {
            if self.contains(&line.start) || self.contains(&line.end) {
                return true;
            }
            // check if the line goes through vertically
            if self.top_left.x < line.start.x
                    && line.start.x < self.bottom_right.x
                        // same for the other point
                    && self.top_left.x < line.end.x
                    && line.end.x < self.bottom_right.x
                    && (
                        // does the line go through the rectangle?
                        (
                            self.top_left.y >= line.start.y
                            && self.bottom_right.y <= line.end.y
                        )
                        // same for the other point
                        ||(
                            self.top_left.y >= line.end.y
                            && self.bottom_right.y <= line.start.y
                        )
                    )
            {
                return true;
            }
            // check if the line goes through horizontally
            if self.top_left.y < line.start.y
                    && line.start.y < self.bottom_right.y
                        // same for the other point
                    && self.top_left.y < line.end.y
                    && line.end.y <  self.bottom_right.y
                    // does the line go through the rectangle?

                    &&(
                        (
                            self.top_left.x >= line.start.x
                            && self.bottom_right.x <= line.end.x
                        )
                    // same for the other point
                        ||(
                            self.top_left.x >= line.end.x
                            && self.bottom_right.x <= line.start.x
                        )
                    )
            {
                return true;
            }
        }
        false
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.top_left.x < point.x
            && point.x < self.bottom_right.x
            && self.top_left.y < point.y
            && point.y < self.bottom_right.y
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

#[cfg(test)]
mod test {
    use crate::{Point, Rect, solve};

    #[test]
    fn example() {
        let ans = solve(
            r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3",
        );
        assert_eq!(ans, 24)
    }

    #[test]
    fn contains() {
        let rect = Rect::new(Point::new(7, 1), Point::new(11, 7));
        assert!(rect.contains(&Point { x: 9, y: 5 }));
    }
}
